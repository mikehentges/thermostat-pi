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


