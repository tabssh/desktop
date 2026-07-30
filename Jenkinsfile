pipeline {
    agent none

    options {
        timeout(time: 60, unit: 'MINUTES')
        disableConcurrentBuilds()
    }

    environment {
        IMAGE = 'casjaysdev/rust:latest'
    }

    stages {
        stage('Lint') {
            agent {
                docker {
                    image env.IMAGE
                    reuseNode true
                }
            }
            steps {
                sh 'cargo fmt --all --check'
                sh 'cargo clippy --workspace --all-targets --all-features --target x86_64-unknown-linux-musl -- -D warnings'
            }
        }

        stage('Test') {
            agent {
                docker {
                    image env.IMAGE
                    reuseNode true
                }
            }
            steps {
                sh 'cargo test --workspace --all-features --target x86_64-unknown-linux-musl'
                sh 'cargo tarpaulin --workspace --all-features --fail-under 60 --timeout 120'
            }
        }

        stage('Build') {
            agent {
                docker {
                    image env.IMAGE
                    reuseNode true
                }
            }
            steps {
                sh '''
                    CRATE=$(grep '^name' Cargo.toml | head -1 | sed 's/name = "\\(.*\\)"/\\1/')
                    cargo build --release --target x86_64-unknown-linux-musl
                    mkdir -p dist
                    cp "target/x86_64-unknown-linux-musl/release/${CRATE}" "dist/${CRATE}-linux-x86_64"
                    sha256sum "dist/${CRATE}-linux-x86_64" > "dist/${CRATE}-linux-x86_64.sha256"
                    # musl's ldd on a genuinely static binary reports "Not a valid
                    # dynamic program" (not glibc's "not a dynamic executable"
                    # wording), so match both phrasings case-insensitively.
                    ldd "target/x86_64-unknown-linux-musl/release/${CRATE}" 2>&1 | grep -qiE "not a dynamic executable|not a valid dynamic program|statically linked"
                '''
            }
            post {
                success {
                    archiveArtifacts artifacts: 'dist/*', fingerprint: true
                }
            }
        }

        stage('Security') {
            parallel {
                stage('Secret Scan') {
                    agent { label 'docker' }
                    steps {
                        script {
                            docker.image('trufflesecurity/trufflehog:latest').inside('--entrypoint=""') {
                                sh 'trufflehog git file://. --only-verified'
                            }
                        }
                    }
                }
                stage('Vulnerability Scan') {
                    agent {
                        docker {
                            image env.IMAGE
                            reuseNode true
                        }
                    }
                    steps {
                        sh 'cargo audit'
                        sh 'cargo deny check licenses advisories bans sources'
                    }
                }
                stage('Image Scan') {
                    agent { label 'docker' }
                    steps {
                        script {
                            if (fileExists('docker/Dockerfile')) {
                                sh 'docker build -f docker/Dockerfile -t scan-target:ci .'
                                docker.image('aquasecurity/trivy:latest').inside('--entrypoint=""') {
                                    sh 'trivy image --severity CRITICAL,HIGH --exit-code 1 scan-target:ci'
                                }
                            } else {
                                echo 'No Dockerfile found, skipping image scan'
                            }
                        }
                    }
                }
            }
        }

        stage('Release') {
            when { tag 'v*' }
            agent {
                docker {
                    image env.IMAGE
                    reuseNode true
                }
            }
            steps {
                sh '''
                    CRATE=$(grep '^name' Cargo.toml | head -1 | sed 's/name = "\\(.*\\)"/\\1/')
                    mkdir -p dist
                    for TARGET in x86_64-unknown-linux-musl aarch64-unknown-linux-musl; do
                        cargo build --release --target "$TARGET"
                        case "$TARGET" in
                            x86_64-unknown-linux-musl)  ARTIFACT="${CRATE}-linux-x86_64" ;;
                            aarch64-unknown-linux-musl) ARTIFACT="${CRATE}-linux-aarch64" ;;
                        esac
                        cp "target/$TARGET/release/$CRATE" "dist/$ARTIFACT"
                        sha256sum "dist/$ARTIFACT" > "dist/$ARTIFACT.sha256"
                    done
                    cargo cyclonedx --format json
                    cp bom.json "dist/${CRATE}-bom.json"
                '''
            }
            post {
                success {
                    archiveArtifacts artifacts: 'dist/*', fingerprint: true
                }
            }
        }
    }
}
