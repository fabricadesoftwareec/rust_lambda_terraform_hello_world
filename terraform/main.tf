# AWS provider configuration.
# Configuración del proveedor AWS.
provider "aws" {
  region = "us-east-1" # Default region: 'us-east-1'.
}

# AWS Lambda function resource.
# Recurso de función AWS Lambda.
resource "aws_lambda_function" "my_lambda_fuction" {
  function_name = "hello_world" # Lambda function name.
  role          = aws_iam_role.my_iam_role.arn # IAM role ARN for the function.

  # Path to the Lambda function's code ZIP file.
  # Ruta al archivo ZIP con el código de la función Lambda.
  filename         = "../rust/target/lambda/rust/bootstrap.zip"
  handler          = "handler" # Lambda function handler.
  runtime          = "provided.al2" # Runtime environment.
  source_code_hash = filebase64sha256("../rust/target/lambda/rust/bootstrap.zip") # ZIP file's SHA256 hash.

  # Place for additional configurations.
  # Espacio para configuraciones adicionales.
}

# IAM role resource.
# Recurso de rol IAM.
resource "aws_iam_role" "my_iam_role" {
  name = "my_iam_role" # IAM role name.

  # IAM role policy definition.
  # Definición de la política del rol IAM.
  assume_role_policy = jsonencode({
    Version = "2012-10-17",
    Statement = [{
      Action = "sts:AssumeRole",
      Effect = "Allow",
      Principal = {
        Service = "lambda.amazonaws.com" # Allows Lambda service to assume this role.
      },
    }]
  })
}
