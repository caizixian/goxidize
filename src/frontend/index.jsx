import React from "react";
import ReactDOM from "react-dom";
import 'bootstrap/dist/css/bootstrap.min.css';

class LinkForm extends React.Component {
    constructor(props) {
        super(props);
        this.initialState = {path: '', destination: 'https://'};
        this.state = this.initialState;

        this.handleChange = this.handleChange.bind(this);
        this.handleSubmit = this.handleSubmit.bind(this);
    }

    handleChange(event) {
        const target = event.target;

        this.setState({
           [target.name]: target.value
        });
    }

    handleSubmit(event) {
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
                    <input type="text" className="form-control" id="path" name="path" value={this.state.path} placeholder="Enter path" onChange={this.handleChange} />
                </div>
                <div className="form-group">
                    <label htmlFor="destination">Destination</label>
                    <input type="text" className="form-control" id="destination" name="destination" value={this.state.destination} placeholder="Enter destination" onChange={this.handleChange} />
                </div>
                <button type="submit" className="btn btn-primary">Submit</button>
            </form>
        );
    }
}

ReactDOM.render(<LinkForm />, document.getElementById('app'));
