<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description>Number To Words Request</description>
   <name>Number To Words Request</name>
   <tag></tag>
   <elementGuidId>603d45a4-ac61-48b8-9850-760490964754</elementGuidId>
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
        &lt;NumberToWords xmlns=&quot;http://www.dataaccess.com/webservicesserver/&quot;>
            &lt;ubiNum>${numToWords}&lt;/ubiNum>
        &lt;/NumberToWords>
    &lt;/Body>
&lt;/Envelope></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>SOAP</soapRequestMethod>
   <soapServiceFunction>NumberToWords</soapServiceFunction>
   <variables>
      <defaultValue>'888888888888'</defaultValue>
      <description></description>
      <id>251673f0-84c8-4041-97ec-15c488636b0b</id>
      <masked>false</masked>
      <name>numToWords</name>
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
   <wsdlAddress>http://www.dataaccess.com/webservicesserver/numberconversion.wso?WSDL</wsdlAddress>
</WebServiceRequestEntity>
