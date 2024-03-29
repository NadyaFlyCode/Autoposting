<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description>{&#xd;
    &quot;jsonrpc&quot;: &quot;2.0&quot;,&#xd;
    &quot;method&quot;: &quot;answer_question&quot;,&#xd;
    &quot;params&quot;: {&#xd;
        &quot;content&quot;: &quot;Hello&quot;&#xd;
    },&#xd;
    &quot;id&quot;: 1&#xd;
}</description>
   <name>New Request</name>
   <tag></tag>
   <elementGuidId>c33a5ca6-9154-4bff-9520-f61b62e18a79</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;jsonrpc\&quot;: \&quot;2.0\&quot;,\n    \&quot;method\&quot;: \&quot;answer_question\&quot;,\n    \&quot;params\&quot;: {\n        \&quot;content\&quot;: \&quot;Напиши исчерпывающую статью-инструкцию на тему Искусственный интеллект. Саму тему писать не надо. Статья должна состоять из 3 подтем. В каждой подтеме должно быть не менее 3 абзацев. Используй связанную структуру, чтобы обеспечить плавный переход между подтемами. Помоги читателю разобраться в теме. Используй примеры и цитаты там, где это необходимо. Важно! Заголовок верни без кавычек.\&quot;,\n        \&quot;description\&quot;: \&quot;Напиши статью\&quot;\n    },\n    \&quot;id\&quot;: 1\n}&quot;,
  &quot;contentType&quot;: &quot;text/plain&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>text/plain</value>
      <webElementGuid>c1a2a6bd-75cd-4c90-9205-cc4c58c96faf</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>9.2.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://sales.ai-chat.itinai.com</restUrl>
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

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
