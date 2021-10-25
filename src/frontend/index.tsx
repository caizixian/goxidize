import * as React from "react";
import * as ReactDOM from "react-dom";
import 'bootstrap/dist/css/bootstrap.min.css';
import * as path from "path";

type Link = {
    path: string,
    destination: string
}

type Empty = Record<any, never>;

class LinkForm extends React.Component<Empty, Link> {
    private initialState = {path: '', destination: 'https://'};
    state: Link;

    constructor(props: Empty) {
        super(props);
        this.state = this.initialState;
        this.handleChange = this.handleChange.bind(this);
        this.handleSubmit = this.handleSubmit.bind(this);
    }

    private readonly handleChange = (event: React.FormEvent<HTMLInputElement>) => {
        const {name, value} = event.currentTarget;
        this.setState({
            [name]: value
        } as Pick<Link, keyof Link>);
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

type LinkTableState = {
    links: Link[]
}

type LinkTableRowProp = {
    link: Link
}

class LinkTableRow extends React.Component<LinkTableRowProp, Empty> {
    constructor(props: LinkTableRowProp) {
        super(props);
    }

    render() {
        return (<tr>
            <td><a href={'/' + this.props.link.path}>{'/' + this.props.link.path}</a></td>
            <td><a href={this.props.link.destination}>{this.props.link.destination}</a></td>
        </tr>);
    }
}

class LinkTable extends React.Component<Empty, LinkTableState> {
    constructor(props: Empty) {
        super(props);
        this.state = {links: []};
    }

    componentDidMount() {
        fetch('/api/v1/link')
            .then(response => response.json())
            .then(
                (result) => {
                    this.setState({links: result});
                },
                (_error) => {
                }
            );
    }

    render() {
        return (<table className="table">
            <thead>
            <tr>
                <th scope="col">Link</th>
                <th scope="col">Destination</th>
            </tr>
            </thead>
            <tbody>
            {this.state.links.map(link => <LinkTableRow link={link}/>)}
            </tbody>
        </table>);
    }
}

class App extends React.Component {
    render() {
        return (
            <div className="container">
                <LinkForm/>
                <LinkTable/>
            </div>
        );
    }
}

ReactDOM.render(<App/>, document.getElementById('app'));
