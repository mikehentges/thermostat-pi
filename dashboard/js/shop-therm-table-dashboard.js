// Initialize the Amazon Cognito credentials provider
AWS.config.region = "us-east-2"; // Region
AWS.config.credentials = new AWS.CognitoIdentityCredentials({
  IdentityPoolId: "us-east-2:a2ab738c-fc99-4fb2-9d73-f6e194e2e980",
});
AWS.config.credentials.get(function (err) {
  if (err) {
    console.log("Error: " + err);
    return;
  }
  console.log("Cognito Identity Id: " + AWS.config.credentials.identityId);
  var cognitoSyncClient = new AWS.CognitoSync();
  cognitoSyncClient.listDatasets(
    {
      IdentityId: AWS.config.credentials.identityId,
      IdentityPoolId: "us-east-2:a2ab738c-fc99-4fb2-9d73-f6e194e2e980",
    },
    function (err, data) {
      if (!err) {
        console.log(JSON.stringify(data));
      }
    }
  );
});

// //you can now check that you can describe the DynamoDB table
// var params = { TableName: tableName };
// var dynamodb = new AWS.DynamoDB({ apiVersion: "2012-08-10" });
// dynamodb.describeTable(params, function (err, data) {
//   console.log(JSON.stringify(data));
// });

var tableName = "Shop_Thermostat";
var hoursBack = 5;
var maxItems = 2000;
const today = new Date();
const yesterday = new Date(new Date().setDate(today.getDate() - 1));

function day_string(date) {
  return (
    date.getFullYear() +
    "-" +
    ("0" + (date.getMonth() + 1)).slice(-2) +
    "-" +
    ("0" + date.getDate()).slice(-2)
  );
}

//Forming the DynamoDB Query
var today_params = {
  TableName: tableName,
  Limit: maxItems,
  ConsistentRead: false,
  ScanIndexForward: true,
  ExpressionAttributeValues: {
    ":reporting_day": day_string(today),
  },
  KeyConditionExpression: "Record_Day = :reporting_day",
};
var yesterday_params = {
  TableName: tableName,
  Limit: maxItems,
  ConsistentRead: false,
  ScanIndexForward: true,
  ExpressionAttributeValues: {
    ":reporting_day": day_string(yesterday),
  },
  KeyConditionExpression: "Record_Day = :reporting_day",
};

function create_table() {
  const body = document.body,
    tbl = document.createElement("table");
  tbl.style.width = "100%";
  tbl.style.border = "1px solid black";
  const header = tbl.insertRow();
  const h1 = header.insertCell();
  h1.appendChild(document.createTextNode("Record_Date"));
  const h2 = header.insertCell();
  h2.appendChild(document.createTextNode("Thermostat_On"));
  const h3 = header.insertCell();
  h3.appendChild(document.createTextNode("Temperature"));

  var docClient = new AWS.DynamoDB.DocumentClient();

  docClient.query(yesterday_params, function (err, data) {
    if (err) console.log(err, err.stack); // an error occurred
    else {
      data.Items.forEach(function (item) {
        const the_date = new Date(item.Record_Date);

        const row = tbl.insertRow();
        const d1 = row.insertCell();
        d1.appendChild(document.createTextNode(the_date.toLocaleString()));
        const d2 = row.insertCell();
        d2.appendChild(document.createTextNode(item.Thermostat_On));
        const d3 = row.insertCell();
        d3.appendChild(document.createTextNode(item.Temperature));
      });
    }
  });
  docClient.query(today_params, function (err, data) {
    if (err) console.log(err, err.stack); // an error occurred
    else {
      data.Items.forEach(function (item) {
        var the_date = new Date(item.Record_Date);
        const row = tbl.insertRow();
        const d1 = row.insertCell();
        d1.appendChild(document.createTextNode(the_date.toLocaleString()));
        const d2 = row.insertCell();
        d2.appendChild(document.createTextNode(item.Thermostat_On));
        const d3 = row.insertCell();
        d3.appendChild(document.createTextNode(item.Temperature));
      });
    }
  });
  body.appendChild(tbl);
}
