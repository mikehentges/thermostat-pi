# Output value definitions

output "invoke_url" {
  value = aws_lambda_function_url.push_temp_function.function_url
}
