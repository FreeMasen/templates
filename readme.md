 # Templates

 This program will take in any template file and a single word and output
 a file with that word inserted into the template.
 
 
 ### Example
 #### Template
 ``` 
 import * as React from 'react'
 
 interface I{}State {
 
 }
 
 interface I{}Props {
 
 }
 
 export default class {} extends React.Component<I{}Props, I{}State> {
     render() {
         return (
             <div>
             </div>
         );
     }
 }
 ```
 #### Command
 `templates NewComponent -t ./path/to/template -o ./path/to/out/dir`
 
 #### output
 ``` typescript
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