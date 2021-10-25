import * as React from "react";
import * as ReactDOM from "react-dom";
import 'bootstrap/dist/css/bootstrap.min.css';

type LinkFormProps = {};
type LinkFormState = {
    path: string,
    destination: string
}

class LinkForm extends React.Component<LinkFormProps, LinkFormState> {
    private initialState = {path: '', destination: 'https://'};
    state: LinkFormState;

    constructor(props: LinkFormProps) {
        super(props);
        this.state = this.initialState;
        this.handleChange = this.handleChange.bind(this);
        this.handleSubmit = this.handleSubmit.bind(this);
    }

    private readonly handleChange = (event: React.FormEvent<HTMLInputElement>) => {
        const {name, value} = event.currentTarget;
        this.setState({
            [name]: value
        } as Pick<LinkFormState, keyof LinkFormState>);
    };

    private readonly handleSubmit = (event: React.FormEvent<HTMLFormElement>) => {
        event.preventDefault();
        if (this.state.destination.startsWith("http") && this.state.path !== "") {
            const options = {
                method: 'POST',
                headers: {'Content-Type': 'application/json'},
                body: JSON.stringify(this.state)
            };
            fetch('/api/v1/link', options)
                .then(_ => this.setState(this.initialState));
        }
    }

    render() {
        return (
            <form onSubmit={this.handleSubmit} className="col-4">
                <div className="form-group">
                    <label htmlFor="path">Path</label>
                    <input type="text" className="form-control" id="path" name="path" value={this.state.path}
                           placeholder="Enter path" onChange={this.handleChange}/>
                </div>
                <div className="form-group">
                    <label htmlFor="destination">Destination</label>
                    <input type="text" className="form-control" id="destination" name="destination"
                           value={this.state.destination} placeholder="Enter destination" onChange={this.handleChange}/>
                </div>
                <button type="submit" className="btn btn-primary">Submit</button>
            </form>
        );
    }
}

ReactDOM.render(<LinkForm/>, document.getElementById('app'));
