use diesel::{PgConnection, QueryDsl};

use crate::{
    error::{LinkError, NodeError},
    links::Link,
    nodes::{
        common::create_epr_node,
        nodes::{ClientNode, EprNode, NodeUsage},
    },
};

impl EprNode {
    pub fn new(conn: &mut PgConnection, name: String) -> Result<EprNode, NodeError> {
        create_epr_node(conn, &name)
    }

    /// Function to get a link between current epr node and passed node
    ///
    /// Function would raise and error if the link does not exist
    pub fn get_link(&self, node: &ClientNode) -> Result<Link, LinkError> {
        let link = Link::get_link(self.id, node.id);
        return link;
    }

    /// Finds the epr node connecting two client nodes.
    ///
    /// Current constraint limits the amount of epr nodes per
    /// client to only 1
    ///
    /// Raise an error if the link does not exist
    pub fn get_epr() -> Result<EprNode, NodeError> {
        todo!()
    }
}

impl NodeUsage for EprNode {
    fn get_id(&self) -> i32 {
        return self.id;
    }

    fn set_in_use(&mut self, value: bool) {
        self.in_use = value;
    }
}
