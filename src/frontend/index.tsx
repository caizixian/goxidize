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
    loadTableData: () => void,
    popUndoStack: () => void,
    undoStack: Link[],
};

class LinkForm extends React.Component<LinkFormProps, LinkFormData> {
    private initialState = {path: '', destination: 'https://'};
    state: LinkFormData;

    constructor(props: LinkFormProps) {
        super(props);
        this.state = this.initialState;
        this.handleChange = this.handleChange.bind(this);
        this.handleSubmit = this.handleSubmit.bind(this);
        this.handleUndo = this.handleUndo.bind(this);
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

    private readonly handleUndo = (event: React.FormEvent<HTMLButtonElement>) => {
        event.preventDefault();
        let link = this.props.undoStack[this.props.undoStack.length - 1];
        if (link !== null) {
            const options = {
                method: 'POST',
                headers: {'Content-Type': 'application/json'},
                body: JSON.stringify({"path": link.path, "destination": link.destination})
            };
            fetch('/api/v1/link', options)
                .then(_ => {
                    this.props.popUndoStack();
                    this.props.loadTableData();
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
                    {this.props.undoStack.length > 0 &&
                    <button className="btn btn-primary m-1" onClick={this.handleUndo}>Undo</button>}
                </div>
            </form>
        );
    }
}

type LinkTableRowProp = {
    link: Link,
    loadTableData: () => void,
    pushUndoStack: (link: Link) => void
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
                this.props.pushUndoStack(this.props.link);
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
    loadTableData: () => void,
    pushUndoStack: (link: Link) => void
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
                                                        loadTableData={this.props.loadTableData}
                                                        pushUndoStack={this.props.pushUndoStack}/>)}
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
        this.popUndoStack = this.popUndoStack.bind(this);
        this.pushUndoStack = this.pushUndoStack.bind(this);
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

    pushUndoStack(link: Link) {
        this.setState((state) => ({undoStack: state.undoStack.concat(link)}));
    }

    popUndoStack() {
        this.setState((state) => ({undoStack: state.undoStack.slice(0, -1)}));
    }

    render() {
        return (
            <div className="container">
                <LinkForm undoStack={this.state.undoStack} popUndoStack={this.popUndoStack}
                          loadTableData={this.loadTableData}/>
                <LinkTable links={this.state.links} loadTableData={this.loadTableData}
                           pushUndoStack={this.pushUndoStack}/>
            </div>
        );
    }
}

ReactDOM.render(<App/>, document.getElementById('app'));
