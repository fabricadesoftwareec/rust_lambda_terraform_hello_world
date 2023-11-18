# rust_lambda_terraform_hello_world
 Rust AWS Lambda Example with Terraform
### English Version

# Rust AWS Lambda Example with Terraform

This repository contains a simple Rust-based AWS Lambda function, demonstrating Infrastructure as Code (IaC) using Terraform. It serves as a guide for programmers looking to quickly start with combining these technologies.

#### Project Structure
- `rust/`: Contains the Rust application.
    - `src/`: Source code.
        - `main.rs`: Main Rust file.
    - `Cargo.lock`: Rust dependencies lock file.
    - `Cargo.toml`: Rust configuration file.
- `terraform/`: Terraform configuration for deploying the Lambda function.
    - `main.tf`: Main Terraform configuration file.
- `Readme.md`: Documentation of the project.

#### Deployment Instructions
To deploy the Lambda function:
1. Navigate to the `rust` directory.
2. Execute the following commands:

bash:

    cargo lambda build --release --output-format zip

    terraform -chdir=../terraform apply

Alternatively, you can run:

    cargo lambda build --release --output-format zip;terraform -chdir=../terraform apply


### Versión en Español

# Ejemplo de AWS Lambda con Rust y Terraform

Este repositorio contiene una función simple de AWS Lambda desarrollada en Rust, demostrando Infraestructura como Código (IaC) usando Terraform. Sirve como guía para programadores que buscan comenzar rápidamente con la combinación de estas tecnologías.

#### Estructura del Proyecto
- `rust/`: Contiene la aplicación en Rust.
    - `src/`: Código fuente.
        - `main.rs`: Archivo principal de Rust.
    - `Cargo.lock`: Archivo de bloqueo de dependencias de Rust.
    - `Cargo.toml`: Archivo de configuración de Rust.
- `terraform/`: Configuración de Terraform para desplegar la función Lambda.
    - `main.tf`: Archivo principal de configuración de Terraform.
- `Readme.md`: Documentación del proyecto.

#### Instrucciones de Despliegue
Para desplegar la función Lambda:
1. Navegue al directorio `rust`.
2. Ejecute los siguientes comandos:

bash:

    cargo lambda build --release --output-format zip

    terraform -chdir=../terraform apply

Alternativamente, puede ejecutar:

    cargo lambda build --release --output-format zip;terraform -chdir=../terraform apply

