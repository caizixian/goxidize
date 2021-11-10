import * as React from "react";
import * as ReactDOM from "react-dom";
import 'bootstrap/dist/css/bootstrap.min.css';

type LinkFormData = {
    path: string,
    destination: string
}

type Link = {
    path: string,
    destination: string,
    id: string,
    created_at: string,
    modified_at: string
}

type Empty = Record<any, never>;

type LinkFormProps = {
    loadTableData: () => void
};

class LinkForm extends React.Component<LinkFormProps, LinkFormData> {
    private initialState = {path: '', destination: 'https://'};
    state: LinkFormData;

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
        } as Pick<LinkFormData, keyof LinkFormData>);
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
                .then(_ => {
                    this.props.loadTableData();
                    this.setState(this.initialState);
                });
        }
    }

    render() {
        return (
            <form onSubmit={this.handleSubmit} className="row align-items-center">
                <label htmlFor="path" className="col-form-label col-3 col-lg-1">Path</label>
                <div className="col-9 col-lg-4">
                    <input type="text" className="form-control m-1" id="path" name="path" value={this.state.path}
                           placeholder="Enter path" onChange={this.handleChange}/>
                </div>
                <label htmlFor="destination" className="col-form-label col-3 col-lg-1">Destination</label>
                <div className="col-9 col-lg-4">
                    <input type="text" className="form-control m-1" id="destination" name="destination"
                           value={this.state.destination} placeholder="Enter destination"
                           onChange={this.handleChange}/>
                </div>
                <div className="col-6 col-lg-1 text-center">
                    <button type="submit" className="btn btn-primary m-1">Submit</button>
                </div>
                <div className="col-6 col-lg-1 text-center">
                    <button className="btn btn-primary m-1">Undo</button>
                </div>
            </form>
        );
    }
}

type LinkTableRowProp = {
    link: Link,
    loadTableData: () => void
}

class LinkTableRow extends React.Component<LinkTableRowProp, Empty> {
    constructor(props: LinkTableRowProp) {
        super(props);
        this.handleOnClick = this.handleOnClick.bind(this);
    }

    handleOnClick() {
        const options = {
            method: 'DELETE'
        };
        fetch(`/api/v1/link/${this.props.link.path}`, options)
            .then(_ => {
                this.props.loadTableData();
            });
    }

    render() {
        return (<tr className="row">
            <td className="col-2 col-lg-1 text-truncate"><a
                href={'/' + this.props.link.path}>{'/' + this.props.link.path}</a></td>
            <td className="col-8 col-lg-10 text-truncate"><a
                href={this.props.link.destination}>{this.props.link.destination}</a></td>
            <td className="col-2 col-lg-1 text-center">
                <button type="button" className="btn btn-danger" onClick={this.handleOnClick}>Delete</button>
            </td>
        </tr>);
    }
}

type LinkTableProps = {
    links: Link[],
    loadTableData: () => void
};

class LinkTable extends React.Component<LinkTableProps, Empty> {
    render() {
        return (<table className="table row">
            <thead>
            <tr className="row">
                <th className="col-2 col-lg-1" scope="col">Link</th>
                <th className="col-8 col-lg-10" scope="col">Destination</th>
                <th className="col-2 col-lg-1" scope="col">Delete</th>
            </tr>
            </thead>
            <tbody>
            {this.props.links.map(link => <LinkTableRow link={link} key={link.id}
                                                        loadTableData={this.props.loadTableData}/>)}
            </tbody>
        </table>);
    }
}

type AppState = {
    links: Link[],
    undoStack: Link[]
}

class App extends React.Component<Empty, AppState> {
    constructor(props: Empty) {
        super(props);
        this.state = {links: [], undoStack: []};
        this.loadTableData = this.loadTableData.bind(this);
    }

    componentDidMount() {
        this.loadTableData()
    }

    loadTableData() {
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
        return (
            <div className="container">
                <LinkForm loadTableData={this.loadTableData}/>
                <LinkTable links={this.state.links} loadTableData={this.loadTableData}/>
            </div>
        );
    }
}

ReactDOM.render(<App/>, document.getElementById('app'));
