<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description>Countries List By Codes</description>
   <name>Countries List</name>
   <tag></tag>
   <elementGuidId>b92cc416-8a86-407b-8635-1fe7e8bd1511</elementGuidId>
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
        &lt;ListOfCountryNamesByCode xmlns=&quot;http://www.oorsprong.org/websamples.countryinfo&quot;/>
    &lt;/Body>
&lt;/Envelope></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>SOAP</soapRequestMethod>
   <soapServiceFunction>ListOfCountryNamesByCode</soapServiceFunction>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()

WS.verifyElementText(response, 'ListOfCountryNamesByCodeResponse.ListOfCountryNamesByCodeResult.tCountryCodeAndName[0].sISOCode', 'AD')
WS.verifyElementText(response, 'ListOfCountryNamesByCodeResponse.ListOfCountryNamesByCodeResult.tCountryCodeAndName[0].sName', 'Andorra')
WS.verifyElementText(response, 'ListOfCountryNamesByCodeResponse.ListOfCountryNamesByCodeResult.tCountryCodeAndName[19].sISOCode', 'BD')
WS.verifyElementText(response, 'ListOfCountryNamesByCodeResponse.ListOfCountryNamesByCodeResult.tCountryCodeAndName[20].sName', 'Belgium')</verificationScript>
   <wsdlAddress>http://webservices.oorsprong.org/websamples.countryinfo/CountryInfoService.wso?WSDL</wsdlAddress>
</WebServiceRequestEntity>
