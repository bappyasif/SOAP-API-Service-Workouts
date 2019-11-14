import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import internal.GlobalVariable as GlobalVariable
import calculatorService.cucumberRunner as cucumberRunner

CucumberKW.runFeatureFile('Include/features/Calculator API/Calculating.feature')

CucumberKW.runFeatureFolder('Include/features/Calculator API')

CucumberKW.runWithCucumberRunner(cucumberRunner.class)

//CucumberKW.runWithCucumberRunner(cucumberRunner.__$stMC)

not_run: addResponse = WS.sendRequest(findTestObject('Calculator API/Add Numbers'))

not_run: println(addResponse)

not_run: String xmlResponseForAddRequest = addResponse.responseBodyContent

not_run: def addResult = new XmlSlurper().parseText(xmlResponseForAddRequest)

not_run: println(addResult)

not_run: devideResponse = WS.sendRequest(findTestObject('Calculator API/Devide Numbers'))

not_run: String xmlResponseForDeviderequest = devideResponse.responseBodyContent

not_run: def devideResult = new XmlSlurper().parseText(xmlResponseForDeviderequest)

not_run: println(devideResult)

not_run: multiplyResponse = WS.sendRequest(findTestObject('Calculator API/Multiply Numbers'))

not_run: String xmlResponseForMultiplyRequest = multiplyResponse.responseBodyContent

not_run: def multiplyResult = new XmlSlurper().parseText(xmlResponseForMultiplyRequest)

not_run: println(multiplyResult)

not_run: subtractResponse = WS.sendRequest(findTestObject('Calculator API/Subtract Numbers'))

not_run: String xmlResponseForSubtractRequest = subtractResponse.responseBodyContent

not_run: def subtractResult = new XmlSlurper().parseText(xmlResponseForSubtractRequest)

not_run: println(subtractResult)

not_run: WS.sendRequest(findTestObject('Calculator API/Add Numbers', [('first_number') : GlobalVariable.First_Number, ('second_number') : GlobalVariable.Second_Number]))

