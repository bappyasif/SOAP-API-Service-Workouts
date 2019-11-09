<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description>Country Flag API Service</description>
   <name>Country Flag</name>
   <tag></tag>
   <elementGuidId>977644dd-a40b-4089-a321-e75fef16a7ef</elementGuidId>
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
        &lt;CountryFlag xmlns=&quot;http://www.oorsprong.org/websamples.countryinfo&quot;>
            &lt;sCountryISOCode>${countryISO}&lt;/sCountryISOCode>
        &lt;/CountryFlag>
    &lt;/Body>
&lt;/Envelope></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>SOAP</soapRequestMethod>
   <soapServiceFunction>CountryFlag</soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.Country_ISOCode</defaultValue>
      <description></description>
      <id>db238c60-c299-41b2-a145-5b5345a763a9</id>
      <masked>false</masked>
      <name>countryISO</name>
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

//WS.verifyElementText(response, 'CountryFlagResponse.CountryFlagResult', 'http://www.oorsprong.org/WebSamples.CountryInfo/Flags/Bangladesh.jpg')
WS.verifyElementText(response, 'CountryFlagResponse.CountryFlagResult', 'http://www.oorsprong.org/WebSamples.CountryInfo/Flags/Vietnam.jpg')</verificationScript>
   <wsdlAddress>http://webservices.oorsprong.org/websamples.countryinfo/CountryInfoService.wso?WSDL</wsdlAddress>
</WebServiceRequestEntity>
