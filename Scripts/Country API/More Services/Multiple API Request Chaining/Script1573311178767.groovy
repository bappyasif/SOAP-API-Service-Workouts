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

languageCodes = WS.sendRequest(findTestObject('Country API/More Services/List Of Language Codes'))

String languageCodesXMLRespomse = languageCodes.responseBodyContent

def languageCodesGeneratedContent = new XmlSlurper().parseText(languageCodesXMLRespomse)

def extractedLanguageISOCode = languageCodesGeneratedContent.ListOfLanguagesByCodeResult.tLanguage[39].sISOCode.text()

println(extractedLanguageISOCode)

//println(languageCode)
languageCode = extractedLanguageISOCode

//GlobalVariable.countryLanguageISOCOde = extractedLanguageISOCode
GlobalVariable.countryLanguageISOCOde = languageCode

codeResponse = WS.sendRequest(findTestObject('Country API/More Services/Language Name', [('languageISO') : GlobalVariable.countryLanguageISOCOde]))

String codeResultingXMLResponse = codeResponse.responseBodyContent

def codeResultGeneratedContent = new XmlSlurper().parseText(codeResultingXMLResponse)

println(codeResultGeneratedContent)

println(lanaguageName)

lanaguageName = codeResultGeneratedContent

//nameResponse = WS.sendRequest(findTestObject('Country API/More Services/Language Code', [('languageName') : codeResultGeneratedContent]))
nameResponse = WS.sendRequest(findTestObject('Country API/More Services/Language Code', [('languageName') : lanaguageName]))

String nameResultingXMLResponse = nameResponse.responseBodyContent

def nameResultGeneratedContent = new XmlSlurper().parseText(nameResultingXMLResponse)

println(nameResultGeneratedContent)

assert nameResultGeneratedContent == languageCode

WS.sendRequestAndVerify(findTestObject('Country API/More Services/List Of Language Codes'))

nestedCodeAPIResponse = WS.sendRequest(findTestObject('Country API/More Services/Language Name', [('languageISO') : nameResultGeneratedContent]))

String nestedCodeXMLResponse = nestedCodeAPIResponse.responseBodyContent

def nestedCodeGenratedContent = new XmlSlurper().parseText(nestedCodeXMLResponse)

println(nestedCodeGenratedContent)

WS.sendRequestAndVerify(findTestObject('Country API/More Services/Language Code', [('languageName') : 'Bengali']))

WS.sendRequestAndVerify(findTestObject('Country API/More Services/Language Name', [('languageISO') : 'eng']))

assert codeResultGeneratedContent == nestedCodeGenratedContent

