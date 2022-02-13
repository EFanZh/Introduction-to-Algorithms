pub mod problems;
pub mod section_22_1_representations_of_graphs;
pub mod section_22_2_breadth_first_search;
pub mod section_22_3_depth_first_search;
pub mod section_22_4_topological_sort;
pub mod section_22_5_strongly_connected_components;

pub trait Edge {
    type Weight;

    fn target(&self) -> usize;
    fn weight(&self) -> &Self::Weight;
}

impl Edge for usize {
    type Weight = ();

    fn target(&self) -> usize {
        *self
    }

    fn weight(&self) -> &Self::Weight {
        &()
    }
}

impl<W> Edge for (usize, W) {
    type Weight = W;

    fn target(&self) -> usize {
        self.0
    }

    fn weight(&self) -> &Self::Weight {
        &self.1
    }
}

pub trait Node {
    type Weight;
    type Edge: Edge;

    fn weight(&self) -> &Self::Weight;
    fn edges(&self) -> &[Self::Edge];
}

impl<E> Node for &[E]
where
    E: Edge,
{
    type Weight = ();
    type Edge = E;

    fn weight(&self) -> &Self::Weight {
        &()
    }

    fn edges(&self) -> &[Self::Edge] {
        self
    }
}

impl<W, E> Node for (W, &[E])
where
    E: Edge,
{
    type Weight = W;
    type Edge = E;

    fn weight(&self) -> &Self::Weight {
        &self.0
    }

    fn edges(&self) -> &[Self::Edge] {
        self.1
    }
}
