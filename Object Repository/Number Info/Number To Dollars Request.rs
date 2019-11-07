<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description>Number To Dollars Request</description>
   <name>Number To Dollars Request</name>
   <tag></tag>
   <elementGuidId>3b2f400b-5526-4f38-a9af-678a24d496ac</elementGuidId>
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
        &lt;NumberToDollars xmlns=&quot;http://www.dataaccess.com/webservicesserver/&quot;>
            &lt;dNum>${numToDollars}&lt;/dNum>
        &lt;/NumberToDollars>
    &lt;/Body>
&lt;/Envelope></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>SOAP</soapRequestMethod>
   <soapServiceFunction>NumberToDollars</soapServiceFunction>
   <variables>
      <defaultValue>'222222222222.22'</defaultValue>
      <description></description>
      <id>0707920b-4173-4659-87a1-c8f14ec122fa</id>
      <masked>false</masked>
      <name>numToDollars</name>
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
