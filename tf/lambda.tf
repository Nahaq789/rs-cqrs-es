resource "aws_lambda_function" "write_api_lambda" {
  function_name = "write-api-lambda"
  role          = aws_iam_role.lambda_iam_role.arn
  package_type  = "Image"
  image_uri     = "${aws_ecr_repository.write_api_repo.repository_url}:latest"

  timeout     = 10
  memory_size = 256

  environment {
    variables = {
      RUST_BACKTRACE = "1"
      RUST_LOG       = "info"
      HOST           = "0.0.0.0"
      PORT           = "8080"
    }
  }
}

resource "aws_lambda_function" "read_api_lambda" {
  function_name = "read-api-lambda"
  role          = aws_iam_role.lambda_iam_role.arn
  package_type  = "Image"
  image_uri     = "${aws_ecr_repository.read_api_repo}.repository_url:latest"

  timeout     = 10
  memory_size = 256

  environment {
    variables = {
      RUST_BACKTRACE = "1"
      RUST_LOG       = "info"
      HOST           = "0.0.0.0"
      PORT           = "8080"
    }
  }
}
