<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description>Multiply Numbers Using SOAP API</description>
   <name>Multiply Numbers</name>
   <tag></tag>
   <elementGuidId>5ecda260-3162-4645-a76b-9c1c1f2fb0a6</elementGuidId>
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
        &lt;Multiply xmlns=&quot;http://tempuri.org/&quot;>
            &lt;intA>${first_number}&lt;/intA>
            &lt;intB>${second_number}&lt;/intB>
        &lt;/Multiply>
    &lt;/Body>
&lt;/Envelope></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>SOAP</soapRequestMethod>
   <soapServiceFunction>Multiply</soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.First_Number</defaultValue>
      <description></description>
      <id>77463579-61ce-4f8c-82b5-31bb98b6d0aa</id>
      <masked>false</masked>
      <name>first_number</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.Second_Number</defaultValue>
      <description></description>
      <id>206ed457-8d83-49ae-834b-1fe49a1c669e</id>
      <masked>false</masked>
      <name>second_number</name>
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
   <wsdlAddress>http://www.dneonline.com/calculator.asmx?wsdl</wsdlAddress>
</WebServiceRequestEntity>
