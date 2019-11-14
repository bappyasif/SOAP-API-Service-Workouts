$(document).ready(function() {var formatter = new CucumberHTML.DOMFormatter($('.cucumber-report'));formatter.uri("Include/features/Calculator API/Calculating.feature");
formatter.feature({
  "name": "Calculating Couple Of Numbers.",
  "description": "",
  "keyword": "Feature",
  "tags": [
    {
      "name": "@tag"
    }
  ]
});
formatter.scenarioOutline({
  "name": "ADD, DEVIDE, MULTIPLY, SUBTRACT",
  "description": "",
  "keyword": "Scenario Outline",
  "tags": [
    {
      "name": "@tag2"
    }
  ]
});
formatter.step({
  "name": "There Are Two Numbers",
  "keyword": "Given "
});
formatter.step({
  "name": "Lets Print Requests Results",
  "keyword": "Then "
});
formatter.examples({
  "name": "",
  "description": "",
  "keyword": "Examples",
  "rows": [
    {
      "cells": [
        "first_number",
        "second_number",
        "Result"
      ]
    },
    {
      "cells": [
        "5",
        "7",
        "success"
      ]
    },
    {
      "cells": [
        "7",
        "9",
        "Fail"
      ]
    }
  ]
});
formatter.scenario({
  "name": "ADD, DEVIDE, MULTIPLY, SUBTRACT",
  "description": "",
  "keyword": "Scenario Outline",
  "tags": [
    {
      "name": "@tag"
    },
    {
      "name": "@tag2"
    }
  ]
});
formatter.step({
  "name": "There Are Two Numbers",
  "keyword": "Given "
});
formatter.match({
  "location": "calculatingNumbers.I_check_for_the_value_in_step()"
});
formatter.result({
  "status": "passed"
});
formatter.step({
  "name": "Lets Print Requests Results",
  "keyword": "Then "
});
formatter.match({
  "location": "calculatingNumbers.printingResults()"
});
formatter.result({
  "status": "passed"
});
formatter.scenario({
  "name": "ADD, DEVIDE, MULTIPLY, SUBTRACT",
  "description": "",
  "keyword": "Scenario Outline",
  "tags": [
    {
      "name": "@tag"
    },
    {
      "name": "@tag2"
    }
  ]
});
formatter.step({
  "name": "There Are Two Numbers",
  "keyword": "Given "
});
formatter.match({
  "location": "calculatingNumbers.I_check_for_the_value_in_step()"
});
formatter.result({
  "status": "passed"
});
formatter.step({
  "name": "Lets Print Requests Results",
  "keyword": "Then "
});
formatter.match({
  "location": "calculatingNumbers.printingResults()"
});
formatter.result({
  "status": "passed"
});
});