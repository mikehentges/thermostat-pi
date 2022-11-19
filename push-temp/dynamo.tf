# aws_dynamodb_table.shop-thermostat-table:
resource "aws_dynamodb_table" "shop-thermostat-table" {
  hash_key       = "Record_Day"
  name           = "Shop_Thermostat"
  range_key      = "Record_Date"
  billing_mode   = "PAY_PER_REQUEST"
  read_capacity  = 0
  write_capacity = 0

  attribute {
    name = "Record_Day"
    type = "S"
  }
  attribute {
    name = "Record_Date"
    type = "S"
  }
}

/*
aws_apigatewayv2_route.push_temp_route
tktt1n58z8
vu8jv9d

aws_apigatewayv2_stage.api_stage
npbg9c

aws_iam_role.push_temp_lambda_execution_role
terraform-20220202023542827000000001

aws_iam_role_policy.write_db_policy
terraform-20220202023542827000000001:lambda_write_db_policy

aws_iam_role_policy_attachment.push_temp_lambda_execution_policy
terraform-20220202023542827000000001-20220202023543185400000002
 # aws_iam_role_policy_attachment.push_temp_lambda_execution_policy will be created
  + resource "aws_iam_role_policy_attachment" "push_temp_lambda_execution_policy" {
      + id         = (known after apply)
      + policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole"
      + role       = "terraform-20220202023542827000000001"


aws_lambda_function.push_temp_lambda

aws_lambda_permission.push_temp_api_permission
terraform-20220202023558884600000003


*/
