import React from "react";

/**
---
category: components
---
@tsProps
*/
@decorator1()
@decorator2(some, args)
class Alert extends React.Component {
  static comoponentId = "Alert";
  static propTypes = {};

  render() {
    return (
      <div>
        {Alert.comoponentId}
      </div>
    );
  }
}
