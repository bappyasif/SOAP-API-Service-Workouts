<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description>Language ISO Code Service</description>
   <name>Language Code</name>
   <tag></tag>
   <elementGuidId>e1980526-0013-4f87-beaf-a80c4b1bd001</elementGuidId>
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
        &lt;LanguageISOCode xmlns=&quot;http://www.oorsprong.org/websamples.countryinfo&quot;>
            &lt;sLanguageName>${languageName}&lt;/sLanguageName>
        &lt;/LanguageISOCode>
    &lt;/Body>
&lt;/Envelope></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>SOAP</soapRequestMethod>
   <soapServiceFunction>LanguageISOCode</soapServiceFunction>
   <variables>
      <defaultValue>'Bengali'</defaultValue>
      <description></description>
      <id>ae016ebf-a4ce-44f0-8a9a-c188c0bef980</id>
      <masked>false</masked>
      <name>languageName</name>
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
</verificationScript>
   <wsdlAddress>http://webservices.oorsprong.org/websamples.countryinfo/CountryInfoService.wso?WSDL</wsdlAddress>
</WebServiceRequestEntity>
