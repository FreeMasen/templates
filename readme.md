 # Templates

 This program will take in a Handlebars template file and prompt you for the values that should be used
 to fill out the template
 
 
 ### Example
 #### Template
 ``` handlebars
 import * as React from 'react'
 
 interface I{{component}}State {
 
 }
 
 interface I{{component}}Props {
 
 }
 
 export default class {{component}} extends React.Component<I{{component}}Props, I{{component}}State> {
     render() {
         return (
             <div>
             </div>
         );
     }
 }
 ```
 #### Command
 ```sh
 $ templates ./path/to/template ./path/to/out/dir
 Please provide the value for {{component}}
 NewComponent
 ```
 
 #### Output
 ``` tsx
import * as React from 'react'
 
 interface INewComponentState {
 
 }
 
 interface INewComponentProps {
 
 }
 
 export default class NewComponent extends React.Component<INewComponentProps, INewComponentState> {
     render() {
         return (
             <div>
             </div>
         );
     }
 }
 ```