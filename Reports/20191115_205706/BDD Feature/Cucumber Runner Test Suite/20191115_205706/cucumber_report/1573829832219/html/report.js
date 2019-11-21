$(document).ready(function() {var formatter = new CucumberHTML.DOMFormatter($('.cucumber-report'));formatter.uri("G:/Katalon Studio/Katalon Workspace/SOAP API Service Workouts/Include/features/CountryInfo/Custom.feature");
formatter.feature({
  "name": "Language Informations",
  "description": "",
  "keyword": "Feature"
});
formatter.scenarioOutline({
  "name": "",
  "description": "",
  "keyword": "Scenario Outline"
});
formatter.step({
  "name": "Language Code Is Provided \u003clanguageCode\u003e",
  "keyword": "Given "
});
formatter.step({
  "name": "Generate Country Language Name \u003clanguageName\u003e",
  "keyword": "Then "
});
formatter.examples({
  "name": "",
  "description": "",
  "keyword": "Examples",
  "rows": [
    {
      "cells": [
        "languageCode",
        "languageName",
        "statusCode"
      ]
    },
    {
      "cells": [
        "ben",
        "Bengali",
        "200"
      ]
    },
    {
      "cells": [
        "bem",
        "Bembi",
        "201"
      ]
    }
  ]
});
formatter.scenario({
  "name": "",
  "description": "",
  "keyword": "Scenario Outline"
});
formatter.step({
  "name": "Language Code Is Provided ben",
  "keyword": "Given "
});
formatter.match({
  "location": "cucumberExample.gemnrateCountryLanguageCode(String)"
});
formatter.result({
  "status": "passed"
});
formatter.step({
  "name": "Generate Country Language Name Bengali",
  "keyword": "Then "
});
formatter.match({
  "location": "cucumberExample.gatherCountryLanguageName(String)"
});
formatter.result({
  "status": "passed"
});
formatter.scenario({
  "name": "",
  "description": "",
  "keyword": "Scenario Outline"
});
formatter.step({
  "name": "Language Code Is Provided bem",
  "keyword": "Given "
});
formatter.match({
  "location": "cucumberExample.gemnrateCountryLanguageCode(String)"
});
formatter.result({
  "status": "passed"
});
formatter.step({
  "name": "Generate Country Language Name Bembi",
  "keyword": "Then "
});
formatter.match({
  "location": "cucumberExample.gatherCountryLanguageName(String)"
});
formatter.result({
  "status": "passed"
});
});