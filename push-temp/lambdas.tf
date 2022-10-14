# PUSH TEMP
data "archive_file" "push_temp_lambda_archive" {
  type = "zip"

  source_file = var.push_temp_bin_path
  output_path = "push_temp_lambda.zip"
}

resource "aws_iam_role" "push_temp_lambda_execution_role" {
  assume_role_policy = <<EOF
{
  "Version": "2012-10-17",
  "Statement": [
    {
      "Action": "sts:AssumeRole",
      "Principal": {
        "Service": "lambda.amazonaws.com"
      },
      "Effect": "Allow",
      "Sid": ""
    }
  ]
}
EOF

}

resource "aws_iam_role_policy_attachment" "push_temp_lambda_execution_policy" {
  role       = aws_iam_role.push_temp_lambda_execution_role.name
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole"
}

resource "aws_lambda_function" "push_temp_lambda" {
  function_name = "PushTemp"

  source_code_hash = data.archive_file.push_temp_lambda_archive.output_base64sha256
  filename         = data.archive_file.push_temp_lambda_archive.output_path

  handler = "func"
  runtime = "provided"
  environment {
    variables = {
      "RUST_LOG" = "debug"
    }
  }

  role = aws_iam_role.push_temp_lambda_execution_role.arn
}

// Add lambda -> DynamoDB policies to the lambda execution role
resource "aws_iam_role_policy" "write_db_policy" {
  name = "lambda_write_db_policy"
  role = aws_iam_role.push_temp_lambda_execution_role.name

  policy = <<EOF
{
  "Version": "2012-10-17",
  "Statement": [
    {
      "Sid": "",
      "Action": [
        "dynamodb:PutItem"
      ],
      "Effect": "Allow",
      "Resource": "arn:aws:dynamodb:us-east-2:376763987559:table/Shop_Thermostat"
    }
  ]
}
EOF
}
