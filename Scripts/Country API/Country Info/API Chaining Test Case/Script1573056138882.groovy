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

println (GlobalVariable.Country_Name)

isocodeListResponse = WS.sendRequest(findTestObject('Country API/Country Info/ISOCodes List'))

String xmlResponseForCodesList = isocodeListResponse.responseBodyContent

def isocodevalueExtracted = new XmlSlurper().parseText(xmlResponseForCodesList)

def country_isoCode = isocodevalueExtracted.ListOfCountryNamesByCodeResult.tCountryCodeAndName[19].sISOCode.text()

println (country_isoCode)

//println (isocodevalueExtracted)


isocodeResponse = WS.sendRequest(findTestObject('Country API/Country Info/Country ISOCode'))

String xmlResponseForISOCode = isocodeResponse.responseBodyContent

def isocodeExtracted = new XmlSlurper().parseText(xmlResponseForISOCode)

println(isocodeExtracted)

println(GlobalVariable.Country_ISOCode)

GlobalVariable.Country_ISOCode = isocodeExtracted



nameResponse = WS.sendRequest(findTestObject('Country API/Country Info/Country Name', [('country_code') : GlobalVariable.Country_ISOCode]))

String xmlResponseForName = nameResponse.responseBodyContent

def nameExtracted = new XmlSlurper().parseText(xmlResponseForName)

println(nameExtracted)

