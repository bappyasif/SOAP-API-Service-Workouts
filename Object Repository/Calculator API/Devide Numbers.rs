<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description>Devide numbers Using SOAP API</description>
   <name>Devide Numbers</name>
   <tag></tag>
   <elementGuidId>3e4f24ed-0246-4aba-bcc3-31ac9b276166</elementGuidId>
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
        &lt;Divide xmlns=&quot;http://tempuri.org/&quot;>
            &lt;intA>${first_number}&lt;/intA>
            &lt;intB>${second_number}&lt;/intB>
        &lt;/Divide>
    &lt;/Body>
&lt;/Envelope></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>SOAP</soapRequestMethod>
   <soapServiceFunction>Divide</soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.First_Number</defaultValue>
      <description></description>
      <id>7b2b4b32-38eb-4eac-8e9e-0e6fc64058e1</id>
      <masked>false</masked>
      <name>first_number</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.Second_Number</defaultValue>
      <description></description>
      <id>09cff45f-2087-4da4-9085-f4236b1540c2</id>
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
