<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Create user</name>
   <tag></tag>
   <elementGuidId>944a14d9-0e4a-4b0a-8625-ecc61312c70f</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;name\&quot;: \&quot;Leanne Graham\&quot;,\n    \&quot;username\&quot;: \&quot;Bret\&quot;,\n    \&quot;email\&quot;: \&quot;Sincere@april.biz\&quot;,\n    \&quot;address\&quot;: {\n      \&quot;street\&quot;: \&quot;Kulas Light\&quot;,\n      \&quot;suite\&quot;: \&quot;Apt. 556\&quot;,\n      \&quot;city\&quot;: \&quot;Gwenborough\&quot;,\n      \&quot;zipcode\&quot;: \&quot;92998-3874\&quot;,\n      \&quot;geo\&quot;: {\n        \&quot;lat\&quot;: \&quot;-37.3159\&quot;,\n        \&quot;lng\&quot;: \&quot;81.1496\&quot;\n      }\n    },\n    \&quot;phone\&quot;: \&quot;1-770-736-8031 x56442\&quot;,\n    \&quot;website\&quot;: \&quot;hildegard.org\&quot;,\n    \&quot;company\&quot;: {\n      \&quot;name\&quot;: \&quot;Romaguera-Crona\&quot;,\n      \&quot;catchPhrase\&quot;: \&quot;Multi-layered client-server neural-net\&quot;,\n      \&quot;bs\&quot;: \&quot;harness real-time e-markets\&quot;\n    }\n  }&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>efe20ac9-62a2-431e-a6bc-ce9aab43f2c2</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.3.5</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>http://jsonplaceholder.typicode.com/users</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()

WS.verifyElementPropertyValue(response, 'id', 11)
WS.verifyElementPropertyValue(response, 'name', 'Leanne Graham')
WS.verifyElementPropertyValue(response, 'username', 'Bret')
WS.verifyElementPropertyValue(response, 'email', 'Sincere@april.biz')
WS.verifyElementPropertyValue(response, 'address.street', 'Kulas Light')
WS.verifyElementPropertyValue(response, 'address.suite', 'Apt. 556')
WS.verifyElementPropertyValue(response, 'address.city', 'Gwenborough')
WS.verifyElementPropertyValue(response, 'address.zipcode', '92998-3874')
WS.verifyElementPropertyValue(response, 'address.geo.lat', '-37.3159')
WS.verifyElementPropertyValue(response, 'address.geo.lng', '81.1496')
WS.verifyElementPropertyValue(response, 'phone', '1-770-736-8031 x56442')
WS.verifyElementPropertyValue(response, 'website', 'hildegard.org')
WS.verifyElementPropertyValue(response, 'company.name', 'Romaguera-Crona')
WS.verifyElementPropertyValue(response, 'company.catchPhrase', 'Multi-layered client-server neural-net')
WS.verifyElementPropertyValue(response, 'company.bs', 'harness real-time e-markets')


WS.verifyResponseStatusCode(response, 201)

assertThat(response.getStatusCode()).isEqualTo(201)</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
