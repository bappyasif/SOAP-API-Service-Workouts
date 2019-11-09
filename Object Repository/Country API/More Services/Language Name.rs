<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description>Country Language API Service</description>
   <name>Language Name</name>
   <tag></tag>
   <elementGuidId>9f137d17-4045-42f1-ac92-4a213556323d</elementGuidId>
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
        &lt;LanguageName xmlns=&quot;http://www.oorsprong.org/websamples.countryinfo&quot;>
            &lt;sISOCode>${languageISO}&lt;/sISOCode>
        &lt;/LanguageName>
    &lt;/Body>
&lt;/Envelope></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>SOAP</soapRequestMethod>
   <soapServiceFunction>LanguageName</soapServiceFunction>
   <variables>
      <defaultValue>'eng'</defaultValue>
      <description></description>
      <id>a76667a4-d060-4b31-87c1-8bd882aeafd0</id>
      <masked>false</masked>
      <name>languageISO</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()

WS.verifyElementText(response, 'LanguageNameResponse.LanguageNameResult', 'English')</verificationScript>
   <wsdlAddress>http://webservices.oorsprong.org/websamples.countryinfo/CountryInfoService.wso?WSDL</wsdlAddress>
</WebServiceRequestEntity>
