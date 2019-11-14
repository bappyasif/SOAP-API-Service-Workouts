<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description>Add Numbers Using SOAP API</description>
   <name>Add Numbers</name>
   <tag></tag>
   <elementGuidId>776ec73a-8071-4740-becb-312edfd713fc</elementGuidId>
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
        &lt;Add xmlns=&quot;http://tempuri.org/&quot;>
            &lt;intA>${first_number}&lt;/intA>
            &lt;intB>${second_number}&lt;/intB>
        &lt;/Add>
    &lt;/Body>
&lt;/Envelope></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>SOAP</soapRequestMethod>
   <soapServiceFunction>Add</soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.First_Number</defaultValue>
      <description></description>
      <id>55d8e45a-153c-427c-a3fd-b0c4c676ef83</id>
      <masked>false</masked>
      <name>first_number</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.Second_Number</defaultValue>
      <description></description>
      <id>08eb7d75-5cf6-42dc-b30b-9a3767f9d9ce</id>
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
