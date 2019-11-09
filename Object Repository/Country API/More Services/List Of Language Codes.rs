<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description>List Of Languages Service</description>
   <name>List Of Language Codes</name>
   <tag></tag>
   <elementGuidId>15261250-aaef-48f0-b2e8-8d3c79d48f0e</elementGuidId>
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
        &lt;ListOfLanguagesByCode xmlns=&quot;http://www.oorsprong.org/websamples.countryinfo&quot;/>
    &lt;/Body>
&lt;/Envelope></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>SOAP</soapRequestMethod>
   <soapServiceFunction>ListOfLanguagesByCode</soapServiceFunction>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()

WS.verifyElementText(response, 'ListOfLanguagesByCodeResponse.ListOfLanguagesByCodeResult.tLanguage[39].sISOCode', 'ben')
//WS.verifyElementText(response, 'ListOfLanguagesByCodeResponse.ListOfLanguagesByCodeResult.tLanguage[47].sISOCode', 'ben')

//WS.verifyElementText(response, 'ListOfLanguagesByCodeResponse.ListOfLanguagesByCodeResult.tLanguage[47]', 'ben')</verificationScript>
   <wsdlAddress>http://webservices.oorsprong.org/websamples.countryinfo/CountryInfoService.wso?WSDL</wsdlAddress>
</WebServiceRequestEntity>
