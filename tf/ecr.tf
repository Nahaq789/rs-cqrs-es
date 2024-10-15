resource "aws_ecr_repository" "write_api_repo" {
  name = "write-api-lambda-repo"
}

resource "aws_ecr_repository" "read_api_repo" {
  name = "read-api-lambda-repo"
}