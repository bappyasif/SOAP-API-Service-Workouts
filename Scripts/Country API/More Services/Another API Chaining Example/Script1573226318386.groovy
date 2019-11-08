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

countriesList = WS.sendRequest(findTestObject('Country API/More Services/Countries List'))

String countriesListXMLResponse = countriesList.responseBodyContent

def countriesListGeneratedContent = new XmlSlurper().parseText(countriesListXMLResponse)

def extractedCode = countriesListGeneratedContent.ListOfCountryNamesByCodeResult.tCountryCodeAndName[19].sISOCode.text()

println(extractedCode)

GlobalVariable.Country_ISOCode = extractedCode

countryFlag = WS.sendRequest(findTestObject('Country API/More Services/Country Flag', [('countryISO') : GlobalVariable.Country_ISOCode]))

String countryFlagXMLResponse = countryFlag.responseBodyContent

def countryFlagLocatedURL = new XmlSlurper().parseText(countryFlagXMLResponse)

println(countryFlagLocatedURL)

WS.sendRequestAndVerify(findTestObject('Country API/More Services/Countries List'))

