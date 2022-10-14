# API
resource "aws_apigatewayv2_api" "thermostat_api" {
  name          = "API"
  description   = "Thermostat API"
  protocol_type = "HTTP"
}

resource "aws_apigatewayv2_stage" "api_stage" {
  api_id      = aws_apigatewayv2_api.thermostat_api.id
  name        = "$default"
  auto_deploy = true
}

resource "aws_apigatewayv2_deployment" "api_deployment" {
  api_id      = aws_apigatewayv2_api.thermostat_api.id
  description = "API deployment"

  triggers = {
    redeployment = sha1(join(",", [
      jsonencode(aws_apigatewayv2_integration.push_temp_integration),
      jsonencode(aws_apigatewayv2_route.push_temp_route),
      ],
    ))
  }

  lifecycle {
    create_before_destroy = true
  }
}

# PUSH TEMP
resource "aws_apigatewayv2_integration" "push_temp_integration" {
  api_id           = aws_apigatewayv2_api.thermostat_api.id
  integration_type = "AWS_PROXY"

  connection_type    = "INTERNET"
  description        = "Push Temp"
  integration_method = "POST"
  integration_uri    = aws_lambda_function.push_temp_lambda.invoke_arn

  payload_format_version = "2.0"
}

resource "aws_apigatewayv2_route" "push_temp_route" {
  api_id    = aws_apigatewayv2_api.thermostat_api.id
  route_key = "POST /push_temp"
  target    = "integrations/${aws_apigatewayv2_integration.push_temp_integration.id}"
}

data "aws_caller_identity" "current" {}

resource "aws_lambda_permission" "push_temp_api_permission" {
  function_name = aws_lambda_function.push_temp_lambda.function_name
  action        = "lambda:InvokeFunction"
  principal     = "apigateway.amazonaws.com"
  source_arn    = "arn:aws:dynamodb:us-east-2:${data.aws_caller_identity.current.account_id}:table/Shop_Thermostat/*/*/${split("/", aws_apigatewayv2_route.push_temp_route.route_key)[1]}"
}
