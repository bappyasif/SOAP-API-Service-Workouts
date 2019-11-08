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

namesList = WS.sendRequest(findTestObject('Country API/More Services/List Of Language Names'))

String namesListXMLResponse = namesList.responseBodyContent

def namesListGeneratedContent = new XmlSlurper().parseText(namesListXMLResponse)

def countryLanguageNameExtracted = namesListGeneratedContent.ListOfLanguagesByNameResult.tLanguage[44].sName.text()

println(countryLanguageNameExtracted)

//assert countryLanguageNameExtracted == languageCodeGeneratedContent
GlobalVariable.countryLanguageISOCOde = countryLanguageNameExtracted

languageName = WS.sendRequest(findTestObject('Country API/More Services/Language Name', [('languageISO') : GlobalVariable.countryLanguageISOCOde]))

String languageNameXMLResponse = languageName.responseBodyContent

def languageNameGeneratedContent = new XmlSlurper().parseText(languageNameXMLResponse)

println(languageNameGeneratedContent)

//assert languageNameGeneratedContent == languageCodeGeneratedContent
languageCode = WS.sendRequest(findTestObject('Country API/More Services/Language Code', [('languageName') : languageNameGeneratedContent]))

String languageCodeXMLResponse = languageCode.responseBodyContent

def languageCodeGeneratedContent = new XmlSlurper().parseText(languageCodeXMLResponse)

println(languageCodeGeneratedContent)

WS.sendRequestAndVerify(findTestObject('Country API/More Services/List Of Language Codes'))

assert languageNameGeneratedContent == countryLanguageNameExtracted