import React from "react";
import {
  HashRouter as Router,
  Route,
  Switch,
  Redirect
} from "react-router-dom";
import Header from "../components/Header";
import Invite from "../components/pages/InviteForm";
import Regist from "../components/pages/RegistForm";
import Handler from "../components/api/Handler";

const pages = () => {
  return (
    <Router>
      <>
        <Header />
        <Switch>
          <Route path="/invite_form" exact component={Invite} />
          <Route path="/regist_form" component={Regist} />
          <Route path="/login" component={Handler} />
          <Route path="/logout" component={Handler} />
          <Route path="/get_user" component={Handler} />
          <Redirect from="*" to="/" />
        </Switch>
      </>
    </Router>
  );
};
export default pages;
