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

numToWordsResponse = WS.sendRequest(findTestObject('Number Info/Number To Words Request', [('numToWords') : '888888888888']))

String xmlResponseForNW = numToWordsResponse.responseBodyContent

def NW_Transformed = new XmlSlurper().parseText(xmlResponseForNW)

println (NW_Transformed)

numToDollarsRespponse = WS.sendRequest(findTestObject('Number Info/Number To Dollars Request', [('numToDollars') : '222222222222.22']))

String xmlresponseForND = numToDollarsRespponse.responseBodyContent

def ND_Transformed = new XmlSlurper().parseText(xmlresponseForND) 

println (ND_Transformed)