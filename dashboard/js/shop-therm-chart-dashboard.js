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

// you can now check that you can describe the DynamoDB table
// var params = { TableName: tableName };
// var dynamodb = new AWS.DynamoDB({ apiVersion: "2012-08-10" });
// dynamodb.describeTable(params, function (err, data) {
//   console.log(JSON.stringify(data));
// });

var tableName = "Shop_Thermostat";
var hoursBack = 5;
var maxItems = 10000;
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

async function create_chart() {
  var docClient = new AWS.DynamoDB.DocumentClient();
  var my_data = [];

  //   docClient.query(today_params, function (err, data) {
  //     if (err) console.log(err, err.stack); // an error occurred
  //     else {
  //       data.Items.forEach(function (item) {
  //         const the_date = new Date(item.Record_Date);
  //         my_data.push({ x: the_date.toISOString(), y: item.Temperature });
  //       });
  //     }
  //   });

  yesterday_promise = await new Promise((resolve, reject) => {
    docClient.query(yesterday_params, (err, data) => {
      if (err) {
        console.log(err);
        resolve(err);
      } else {
        console.log("success - got data");
        data.Items.forEach((item) => {
          const the_date = new Date(item.Record_Date);
          my_data.push({ x: the_date.getTime(), y: item.Temperature });
        });
        resolve(data);
      }
    });
  });
  today_promise = await new Promise((resolve, reject) => {
    docClient.query(today_params, (err, data) => {
      if (err) {
        console.log(err);
        resolve(err);
      } else {
        console.log("success - got data");
        data.Items.forEach((item) => {
          const the_date = new Date(item.Record_Date);
          my_data.push({ x: the_date.getTime(), y: item.Temperature });
        });
        resolve(data);
      }
    });
  });

  var average_data = [];
  for (var i in my_data) {
    //console.log("i is: " + i);
    if (i < 10 || i > my_data.length - 10) {
      continue;
    } else {
      var sum_temp = 0;
      var sum_date = 0;
      var min = Number(i) - 10;
      var max = 10 + Number(i);
      //console.log("min is: " + min + " max is: " + max);
      for (let j = min; j < max; j++) {
        //console.log("j is: " + j + " and i is: " + i);
        sum_date += my_data[j].x;
        sum_temp += my_data[j].y;
      }
      average_data.push({ x: new Date(sum_date / 20.0), y: sum_temp / 20.0 });
    }
  }

  const input_data = {
    datasets: [
      {
        label: "Moving-Average Temperature",
        backgroundColor: "rgb(50, 168, 82)",
        borderColor: "rgb(50, 168, 82)",
        //        data: my_data,
        data: average_data,
        cubicInterpolationMode: "monotone",
      },
      //   {
      //     label: "Shop Temperature",
      //     backgroundColor: "rgb(255, 99, 132)",
      //     borderColor: "rgb(255, 99, 132)",
      //     data: my_data,
      //     cubicInterpolationMode: "monotone",
      //   },
    ],
  };

  const config = {
    type: "line",
    data: input_data,
    options: {
      scales: {
        x: {
          parsing: false,
          type: "time",
          time: {
            unit: "hour",
          },
        },
      },
    },
  };
  console.log(config);
  const myChart = new Chart(document.getElementById("canvas"), config);
}
