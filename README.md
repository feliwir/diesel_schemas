# Diesel Scoped

Diesel makes it easy to deal with different tables in a single database, however for multi-tenancy approaches
it's often useful to split content into different schemes (apart from the default scheme).

This is currently not handled at all by the Diesel library. This library extends the default functionality
with an easy scheme switching.