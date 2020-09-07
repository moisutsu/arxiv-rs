/// Macro that allows you to easily build ArxivQuery.
/// # Example
/// ```rust
/// use arxiv::query;
///
/// // query type is ArxivQuery
/// let query = query!(
///     search_query = "cat:cs.CL",
///     start = 0,
///     max_results = 5,
///     sort_by = "submittedDate",
///     sort_order = "descending"
/// );
/// ```
#[macro_export]
macro_rules! query {
    ( $($i:ident = $e:expr),* ) => {
        {
            let temp_query = $crate::ArxivQueryBuilder::new();
            $(
                $crate::query!(@inner, $i, $e, temp_query);
            )*
            temp_query.build()
        }
    };

    (@inner, search_query, $e:expr, $temp_query:ident) => {
        let $temp_query = $temp_query.search_query($e);
    };

    (@inner, id_list, $e:expr, $temp_query:ident) => {
        let $temp_query = $temp_query.id_list($e);
    };

    (@inner, start, $e:expr, $temp_query:ident) => {
        let $temp_query = $temp_query.start($e);
    };

    (@inner, max_results, $e:expr, $temp_query:ident) => {
        let $temp_query = $temp_query.max_results($e);
    };

    (@inner, sort_by, $e:expr, $temp_query:ident) => {
        let $temp_query = $temp_query.sort_by($e);
    };

    (@inner, sort_order, $e:expr, $temp_query:ident) => {
        let $temp_query = $temp_query.sort_order($e);
    };
}
