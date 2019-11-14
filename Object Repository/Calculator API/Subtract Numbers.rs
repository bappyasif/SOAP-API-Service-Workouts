<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description>Subtract Numbers Using SOAP API</description>
   <name>Subtract Numbers</name>
   <tag></tag>
   <elementGuidId>69de42eb-1e0d-4989-816a-771df10e46ae</elementGuidId>
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
        &lt;Subtract xmlns=&quot;http://tempuri.org/&quot;>
            &lt;intA>${first_number}&lt;/intA>
            &lt;intB>${second_number}&lt;/intB>
        &lt;/Subtract>
    &lt;/Body>
&lt;/Envelope></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>SOAP</soapRequestMethod>
   <soapServiceFunction>Subtract</soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.First_Number</defaultValue>
      <description></description>
      <id>6332e99e-6154-4d77-877b-ae8c0cd03bd8</id>
      <masked>false</masked>
      <name>first_number</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.Second_Number</defaultValue>
      <description></description>
      <id>43be996b-473e-4144-a93f-e04d93ec4dc0</id>
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
