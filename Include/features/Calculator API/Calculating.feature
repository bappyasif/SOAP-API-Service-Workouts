#Author: your.email@your.domain.com
#Keywords Summary :
#Feature: List of scenarios.
#Scenario: Business rule through list of steps with arguments.
#Given: Some precondition step
#When: Some key actions
#Then: To observe outcomes or validation
#And,But: To enumerate more Given,When,Then steps
#Scenario Outline: List of steps for data-driven as an Examples and <placeholder>
#Examples: Container for s table
#Background: List of steps run before each of the scenarios
#""" (Doc Strings)
#| (Data Tables)
#@ (Tags/Labels):To group Scenarios
#<> (placeholder)
#""
## (Comments)
#Sample Feature Definition Template
@tag
Feature: Calculating Couple Of Numbers.

  @tag2
  Scenario Outline: ADD, DEVIDE, MULTIPLY, SUBTRACT
    Given There Are Two Numbers
     
    Then Lets Print Requests Results

    Examples: 
      | first_number | second_number | Result  |
      |            5 |             7 | success |
      |            7 |             9 | Fail    |
