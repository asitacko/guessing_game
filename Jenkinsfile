pipeline {
    agent any
    stages {
        stage('Install') {
            steps {
                echo "check1"
                sh "curl https://sh.rustup.rs -sSf >> ../rust.sh"
                echo "check2"
                sh "chmod 555 ../rust.sh"
                echo "chmod"
                sh "../rust.sh -y"
                ech0 "check22"
                sh "rustup install stable"
                echo "check3"
                sh "rustup install nightly"
                echo "check4"
                sh "rustup default nightly"
                echo "check5"
            }
        }
        stage('Build') {
            steps {
                echo "build5"
                sh "cargo build"
            }
        }
    }
}
