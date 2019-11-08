<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description>List Of Language Names</description>
   <name>List Of Language Names</name>
   <tag></tag>
   <elementGuidId>79f4ea65-7425-4682-b6c9-598bd73b8400</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <restRequestMethod></restRequestMethod>
   <restUrl></restUrl>
   <serviceType>SOAP</serviceType>
   <soapBody>&lt;Envelope xmlns=&quot;http://schemas.xmlsoap.org/soap/envelope/&quot;>
    &lt;Body>
        &lt;ListOfLanguagesByName xmlns=&quot;http://www.oorsprong.org/websamples.countryinfo&quot;/>
    &lt;/Body>
&lt;/Envelope></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>SOAP</soapRequestMethod>
   <soapServiceFunction>ListOfLanguagesByName</soapServiceFunction>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()

WS.verifyElementText(response, 'ListOfLanguagesByNameResponse.ListOfLanguagesByNameResult.tLanguage[104]', 'eng')
WS.verifyElementText(response, 'ListOfLanguagesByNameResponse.ListOfLanguagesByNameResult.tLanguage[3]', 'Acoli')
WS.verifyElementText(response, 'ListOfLanguagesByNameResponse.ListOfLanguagesByNameResult.tLanguage[2].sName', 'ach')
WS.verifyElementText(response, 'ListOfLanguagesByNameResponse.ListOfLanguagesByNameResult.tLanguage[14].sName', 'alg')
WS.verifyElementText(response, 'ListOfLanguagesByNameResponse.ListOfLanguagesByNameResult.tLanguage[15]', 'Algonquian languages')

//WS.verifyElementText(response, 'ListOfLanguagesByNameResponse.ListOfLanguagesByNameResult.tLanguage[150].sName', 'Ga')
WS.verifyElementText(response, 'ListOfLanguagesByNameResponse.ListOfLanguagesByNameResult.tLanguage[44]', 'ben')
WS.verifyElementText(response, 'ListOfLanguagesByNameResponse.ListOfLanguagesByNameResult.tLanguage[65]', 'bul')</verificationScript>
   <wsdlAddress>http://webservices.oorsprong.org/websamples.countryinfo/CountryInfoService.wso?WSDL</wsdlAddress>
</WebServiceRequestEntity>
