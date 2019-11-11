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

println(countryISO)

println(languageCode)

println(languageName)

flagResponse = WS.sendRequest(findTestObject('Country API/More Services/Country Flag - Data Driven', [('countryISO') : countryISO]))

//println (countryISO)
WS.sendRequestAndVerify(findTestObject('Country API/More Services/Country Flag - Data Driven'))

codeResponse = WS.sendRequest(findTestObject('Country API/More Services/Language Code', [('languageName') : languageName]))

//println (languageName)
WS.sendRequestAndVerify(findTestObject('Country API/More Services/Language Code', [('languageName') : 'Bengali']))

nameResponse = WS.sendRequest(findTestObject('Country API/More Services/Language Name', [('languageCode') : languageCode]))

//println (languageCode)
WS.sendRequestAndVerify(findTestObject('Country API/More Services/Language Name', [('languageISO') : 'eng']))

CustomKeywords.'customKeywords.ExampleCustomKeyWord.printLanguage'(languageName)

CustomKeywords.'customKeywords.ExampleCustomKeyWord.printCode'(languageCode)

CustomKeywords.'customKeywords.ExampleCustomKeyWord.verifyStatusCode'(findTestObject('Country API/More Services/Countries List'), 
    200)

