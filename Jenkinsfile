pipeline {
  agent any
  stages {
    stage('Setup') {
      steps {
        sh 'curl https://sh.rustup.rs -sSf | sh -s -- -y'
      }
    }
    stage('Build') {
      steps {
        sh 'cargo build'
      }
    }
  }
}