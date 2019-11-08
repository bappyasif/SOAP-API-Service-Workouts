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

codeList = WS.sendRequest(findTestObject('Country API/Country Info/ISOCodes List'))

println(GlobalVariable.Country_ISOCode)

String xmlResponseForCodes = codeList.responseBodyContent

def allCountryIsoCodes = new XmlSlurper().parseText(xmlResponseForCodes)

def extractedCode = allCountryIsoCodes.ListOfCountryNamesByCodeResult.tCountryCodeAndName[19].sISOCode.text()

println(extractedCode)

GlobalVariable.Country_ISOCode = extractedCode

countryName = WS.sendRequest(findTestObject('Country API/Country Info/Country Name', [('country_code') : GlobalVariable.Country_ISOCode]))

String xmlResponseForName = countryName.responseBodyContent

def nameExtracted = new XmlSlurper().parseText(xmlResponseForName)

println(nameExtracted)

println(GlobalVariable.Country_Name)

countryCode = WS.sendRequest(findTestObject('Country API/Country Info/Country ISOCode', [('countryName') : nameExtracted]))

String xmlResponseForCountry = countryCode.responseBodyContent

def extractedName = new XmlSlurper().parseText(xmlResponseForCountry)

println(extractedName)

GlobalVariable.Country_Name = nameExtracted

println(GlobalVariable.Country_Name)

countriesList = WS.sendRequest(findTestObject('Country API/Country Info/Countries List'))

String xmlResponseForCNames = countriesList.responseBodyContent

def cnamesList = new XmlSlurper().parseText(xmlResponseForCNames)

def nameCheck = cnamesList.ListOfCountryNamesByNameResult.tCountryCodeAndName[18].sName.text()

println('Name Check : ' + nameCheck)

def codeCheck = cnamesList.ListOfCountryNamesByNameResult.tCountryCodeAndName[18].sISOCode.text()

println('Code Check : ' + codeCheck)

WS.sendRequestAndVerify(findTestObject('Country API/Country Info/Country ISOCode', [('countryName') : nameCheck]))

