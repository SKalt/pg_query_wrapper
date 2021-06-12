#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ParseResult {
    #[prost(int32, tag="1")]
    pub version: i32,
    #[prost(message, repeated, tag="2")]
    pub stmts: ::prost::alloc::vec::Vec<RawStmt>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScanResult {
    #[prost(int32, tag="1")]
    pub version: i32,
    #[prost(message, repeated, tag="2")]
    pub tokens: ::prost::alloc::vec::Vec<ScanToken>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Node {
    #[prost(oneof="node::Node", tags="1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95, 96, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122, 123, 124, 125, 126, 127, 128, 129, 130, 131, 132, 133, 134, 135, 136, 137, 138, 139, 140, 141, 142, 143, 144, 145, 146, 147, 148, 149, 150, 151, 152, 153, 154, 155, 156, 157, 158, 159, 160, 161, 162, 163, 164, 165, 166, 167, 168, 169, 170, 171, 172, 173, 174, 175, 176, 177, 178, 179, 180, 181, 182, 183, 184, 185, 186, 187, 188, 189, 190, 191, 192, 193, 194, 195, 196, 197, 198, 199, 200, 201, 202, 203, 204, 205, 206, 207, 208, 209, 210, 211, 212, 213, 214, 215, 216, 217, 218, 219, 220, 221, 222, 223, 224, 225, 226, 227, 228")]
    pub node: ::core::option::Option<node::Node>,
}
/// Nested message and enum types in `Node`.
pub mod node {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Node {
        #[prost(message, tag="1")]
        Alias(super::Alias),
        #[prost(message, tag="2")]
        RangeVar(super::RangeVar),
        #[prost(message, tag="3")]
        TableFunc(::prost::alloc::boxed::Box<super::TableFunc>),
        #[prost(message, tag="4")]
        Expr(super::Expr),
        #[prost(message, tag="5")]
        Var(::prost::alloc::boxed::Box<super::Var>),
        #[prost(message, tag="6")]
        Param(::prost::alloc::boxed::Box<super::Param>),
        #[prost(message, tag="7")]
        Aggref(::prost::alloc::boxed::Box<super::Aggref>),
        #[prost(message, tag="8")]
        GroupingFunc(::prost::alloc::boxed::Box<super::GroupingFunc>),
        #[prost(message, tag="9")]
        WindowFunc(::prost::alloc::boxed::Box<super::WindowFunc>),
        #[prost(message, tag="10")]
        SubscriptingRef(::prost::alloc::boxed::Box<super::SubscriptingRef>),
        #[prost(message, tag="11")]
        FuncExpr(::prost::alloc::boxed::Box<super::FuncExpr>),
        #[prost(message, tag="12")]
        NamedArgExpr(::prost::alloc::boxed::Box<super::NamedArgExpr>),
        #[prost(message, tag="13")]
        OpExpr(::prost::alloc::boxed::Box<super::OpExpr>),
        #[prost(message, tag="14")]
        DistinctExpr(::prost::alloc::boxed::Box<super::DistinctExpr>),
        #[prost(message, tag="15")]
        NullIfExpr(::prost::alloc::boxed::Box<super::NullIfExpr>),
        #[prost(message, tag="16")]
        ScalarArrayOpExpr(::prost::alloc::boxed::Box<super::ScalarArrayOpExpr>),
        #[prost(message, tag="17")]
        BoolExpr(::prost::alloc::boxed::Box<super::BoolExpr>),
        #[prost(message, tag="18")]
        SubLink(::prost::alloc::boxed::Box<super::SubLink>),
        #[prost(message, tag="19")]
        SubPlan(::prost::alloc::boxed::Box<super::SubPlan>),
        #[prost(message, tag="20")]
        AlternativeSubPlan(::prost::alloc::boxed::Box<super::AlternativeSubPlan>),
        #[prost(message, tag="21")]
        FieldSelect(::prost::alloc::boxed::Box<super::FieldSelect>),
        #[prost(message, tag="22")]
        FieldStore(::prost::alloc::boxed::Box<super::FieldStore>),
        #[prost(message, tag="23")]
        RelabelType(::prost::alloc::boxed::Box<super::RelabelType>),
        #[prost(message, tag="24")]
        CoerceViaIo(::prost::alloc::boxed::Box<super::CoerceViaIo>),
        #[prost(message, tag="25")]
        ArrayCoerceExpr(::prost::alloc::boxed::Box<super::ArrayCoerceExpr>),
        #[prost(message, tag="26")]
        ConvertRowtypeExpr(::prost::alloc::boxed::Box<super::ConvertRowtypeExpr>),
        #[prost(message, tag="27")]
        CollateExpr(::prost::alloc::boxed::Box<super::CollateExpr>),
        #[prost(message, tag="28")]
        CaseExpr(::prost::alloc::boxed::Box<super::CaseExpr>),
        #[prost(message, tag="29")]
        CaseWhen(::prost::alloc::boxed::Box<super::CaseWhen>),
        #[prost(message, tag="30")]
        CaseTestExpr(::prost::alloc::boxed::Box<super::CaseTestExpr>),
        #[prost(message, tag="31")]
        ArrayExpr(::prost::alloc::boxed::Box<super::ArrayExpr>),
        #[prost(message, tag="32")]
        RowExpr(::prost::alloc::boxed::Box<super::RowExpr>),
        #[prost(message, tag="33")]
        RowCompareExpr(::prost::alloc::boxed::Box<super::RowCompareExpr>),
        #[prost(message, tag="34")]
        CoalesceExpr(::prost::alloc::boxed::Box<super::CoalesceExpr>),
        #[prost(message, tag="35")]
        MinMaxExpr(::prost::alloc::boxed::Box<super::MinMaxExpr>),
        #[prost(message, tag="36")]
        SqlvalueFunction(::prost::alloc::boxed::Box<super::SqlValueFunction>),
        #[prost(message, tag="37")]
        XmlExpr(::prost::alloc::boxed::Box<super::XmlExpr>),
        #[prost(message, tag="38")]
        NullTest(::prost::alloc::boxed::Box<super::NullTest>),
        #[prost(message, tag="39")]
        BooleanTest(::prost::alloc::boxed::Box<super::BooleanTest>),
        #[prost(message, tag="40")]
        CoerceToDomain(::prost::alloc::boxed::Box<super::CoerceToDomain>),
        #[prost(message, tag="41")]
        CoerceToDomainValue(::prost::alloc::boxed::Box<super::CoerceToDomainValue>),
        #[prost(message, tag="42")]
        SetToDefault(::prost::alloc::boxed::Box<super::SetToDefault>),
        #[prost(message, tag="43")]
        CurrentOfExpr(::prost::alloc::boxed::Box<super::CurrentOfExpr>),
        #[prost(message, tag="44")]
        NextValueExpr(::prost::alloc::boxed::Box<super::NextValueExpr>),
        #[prost(message, tag="45")]
        InferenceElem(::prost::alloc::boxed::Box<super::InferenceElem>),
        #[prost(message, tag="46")]
        TargetEntry(::prost::alloc::boxed::Box<super::TargetEntry>),
        #[prost(message, tag="47")]
        RangeTblRef(super::RangeTblRef),
        #[prost(message, tag="48")]
        JoinExpr(::prost::alloc::boxed::Box<super::JoinExpr>),
        #[prost(message, tag="49")]
        FromExpr(::prost::alloc::boxed::Box<super::FromExpr>),
        #[prost(message, tag="50")]
        OnConflictExpr(::prost::alloc::boxed::Box<super::OnConflictExpr>),
        #[prost(message, tag="51")]
        IntoClause(::prost::alloc::boxed::Box<super::IntoClause>),
        #[prost(message, tag="52")]
        RawStmt(::prost::alloc::boxed::Box<super::RawStmt>),
        #[prost(message, tag="53")]
        Query(::prost::alloc::boxed::Box<super::Query>),
        #[prost(message, tag="54")]
        InsertStmt(::prost::alloc::boxed::Box<super::InsertStmt>),
        #[prost(message, tag="55")]
        DeleteStmt(::prost::alloc::boxed::Box<super::DeleteStmt>),
        #[prost(message, tag="56")]
        UpdateStmt(::prost::alloc::boxed::Box<super::UpdateStmt>),
        #[prost(message, tag="57")]
        SelectStmt(::prost::alloc::boxed::Box<super::SelectStmt>),
        #[prost(message, tag="58")]
        AlterTableStmt(super::AlterTableStmt),
        #[prost(message, tag="59")]
        AlterTableCmd(::prost::alloc::boxed::Box<super::AlterTableCmd>),
        #[prost(message, tag="60")]
        AlterDomainStmt(::prost::alloc::boxed::Box<super::AlterDomainStmt>),
        #[prost(message, tag="61")]
        SetOperationStmt(::prost::alloc::boxed::Box<super::SetOperationStmt>),
        #[prost(message, tag="62")]
        GrantStmt(super::GrantStmt),
        #[prost(message, tag="63")]
        GrantRoleStmt(super::GrantRoleStmt),
        #[prost(message, tag="64")]
        AlterDefaultPrivilegesStmt(super::AlterDefaultPrivilegesStmt),
        #[prost(message, tag="65")]
        ClosePortalStmt(super::ClosePortalStmt),
        #[prost(message, tag="66")]
        ClusterStmt(super::ClusterStmt),
        #[prost(message, tag="67")]
        CopyStmt(::prost::alloc::boxed::Box<super::CopyStmt>),
        #[prost(message, tag="68")]
        CreateStmt(super::CreateStmt),
        #[prost(message, tag="69")]
        DefineStmt(super::DefineStmt),
        #[prost(message, tag="70")]
        DropStmt(super::DropStmt),
        #[prost(message, tag="71")]
        TruncateStmt(super::TruncateStmt),
        #[prost(message, tag="72")]
        CommentStmt(::prost::alloc::boxed::Box<super::CommentStmt>),
        #[prost(message, tag="73")]
        FetchStmt(super::FetchStmt),
        #[prost(message, tag="74")]
        IndexStmt(::prost::alloc::boxed::Box<super::IndexStmt>),
        #[prost(message, tag="75")]
        CreateFunctionStmt(super::CreateFunctionStmt),
        #[prost(message, tag="76")]
        AlterFunctionStmt(super::AlterFunctionStmt),
        #[prost(message, tag="77")]
        DoStmt(super::DoStmt),
        #[prost(message, tag="78")]
        RenameStmt(::prost::alloc::boxed::Box<super::RenameStmt>),
        #[prost(message, tag="79")]
        RuleStmt(::prost::alloc::boxed::Box<super::RuleStmt>),
        #[prost(message, tag="80")]
        NotifyStmt(super::NotifyStmt),
        #[prost(message, tag="81")]
        ListenStmt(super::ListenStmt),
        #[prost(message, tag="82")]
        UnlistenStmt(super::UnlistenStmt),
        #[prost(message, tag="83")]
        TransactionStmt(super::TransactionStmt),
        #[prost(message, tag="84")]
        ViewStmt(::prost::alloc::boxed::Box<super::ViewStmt>),
        #[prost(message, tag="85")]
        LoadStmt(super::LoadStmt),
        #[prost(message, tag="86")]
        CreateDomainStmt(::prost::alloc::boxed::Box<super::CreateDomainStmt>),
        #[prost(message, tag="87")]
        CreatedbStmt(super::CreatedbStmt),
        #[prost(message, tag="88")]
        DropdbStmt(super::DropdbStmt),
        #[prost(message, tag="89")]
        VacuumStmt(super::VacuumStmt),
        #[prost(message, tag="90")]
        ExplainStmt(::prost::alloc::boxed::Box<super::ExplainStmt>),
        #[prost(message, tag="91")]
        CreateTableAsStmt(::prost::alloc::boxed::Box<super::CreateTableAsStmt>),
        #[prost(message, tag="92")]
        CreateSeqStmt(super::CreateSeqStmt),
        #[prost(message, tag="93")]
        AlterSeqStmt(super::AlterSeqStmt),
        #[prost(message, tag="94")]
        VariableSetStmt(super::VariableSetStmt),
        #[prost(message, tag="95")]
        VariableShowStmt(super::VariableShowStmt),
        #[prost(message, tag="96")]
        DiscardStmt(super::DiscardStmt),
        #[prost(message, tag="97")]
        CreateTrigStmt(::prost::alloc::boxed::Box<super::CreateTrigStmt>),
        #[prost(message, tag="98")]
        CreatePlangStmt(super::CreatePLangStmt),
        #[prost(message, tag="99")]
        CreateRoleStmt(super::CreateRoleStmt),
        #[prost(message, tag="100")]
        AlterRoleStmt(super::AlterRoleStmt),
        #[prost(message, tag="101")]
        DropRoleStmt(super::DropRoleStmt),
        #[prost(message, tag="102")]
        LockStmt(super::LockStmt),
        #[prost(message, tag="103")]
        ConstraintsSetStmt(super::ConstraintsSetStmt),
        #[prost(message, tag="104")]
        ReindexStmt(super::ReindexStmt),
        #[prost(message, tag="105")]
        CheckPointStmt(super::CheckPointStmt),
        #[prost(message, tag="106")]
        CreateSchemaStmt(super::CreateSchemaStmt),
        #[prost(message, tag="107")]
        AlterDatabaseStmt(super::AlterDatabaseStmt),
        #[prost(message, tag="108")]
        AlterDatabaseSetStmt(super::AlterDatabaseSetStmt),
        #[prost(message, tag="109")]
        AlterRoleSetStmt(super::AlterRoleSetStmt),
        #[prost(message, tag="110")]
        CreateConversionStmt(super::CreateConversionStmt),
        #[prost(message, tag="111")]
        CreateCastStmt(super::CreateCastStmt),
        #[prost(message, tag="112")]
        CreateOpClassStmt(super::CreateOpClassStmt),
        #[prost(message, tag="113")]
        CreateOpFamilyStmt(super::CreateOpFamilyStmt),
        #[prost(message, tag="114")]
        AlterOpFamilyStmt(super::AlterOpFamilyStmt),
        #[prost(message, tag="115")]
        PrepareStmt(::prost::alloc::boxed::Box<super::PrepareStmt>),
        #[prost(message, tag="116")]
        ExecuteStmt(super::ExecuteStmt),
        #[prost(message, tag="117")]
        DeallocateStmt(super::DeallocateStmt),
        #[prost(message, tag="118")]
        DeclareCursorStmt(::prost::alloc::boxed::Box<super::DeclareCursorStmt>),
        #[prost(message, tag="119")]
        CreateTableSpaceStmt(super::CreateTableSpaceStmt),
        #[prost(message, tag="120")]
        DropTableSpaceStmt(super::DropTableSpaceStmt),
        #[prost(message, tag="121")]
        AlterObjectDependsStmt(::prost::alloc::boxed::Box<super::AlterObjectDependsStmt>),
        #[prost(message, tag="122")]
        AlterObjectSchemaStmt(::prost::alloc::boxed::Box<super::AlterObjectSchemaStmt>),
        #[prost(message, tag="123")]
        AlterOwnerStmt(::prost::alloc::boxed::Box<super::AlterOwnerStmt>),
        #[prost(message, tag="124")]
        AlterOperatorStmt(super::AlterOperatorStmt),
        #[prost(message, tag="125")]
        AlterTypeStmt(super::AlterTypeStmt),
        #[prost(message, tag="126")]
        DropOwnedStmt(super::DropOwnedStmt),
        #[prost(message, tag="127")]
        ReassignOwnedStmt(super::ReassignOwnedStmt),
        #[prost(message, tag="128")]
        CompositeTypeStmt(super::CompositeTypeStmt),
        #[prost(message, tag="129")]
        CreateEnumStmt(super::CreateEnumStmt),
        #[prost(message, tag="130")]
        CreateRangeStmt(super::CreateRangeStmt),
        #[prost(message, tag="131")]
        AlterEnumStmt(super::AlterEnumStmt),
        #[prost(message, tag="132")]
        AlterTsdictionaryStmt(super::AlterTsDictionaryStmt),
        #[prost(message, tag="133")]
        AlterTsconfigurationStmt(super::AlterTsConfigurationStmt),
        #[prost(message, tag="134")]
        CreateFdwStmt(super::CreateFdwStmt),
        #[prost(message, tag="135")]
        AlterFdwStmt(super::AlterFdwStmt),
        #[prost(message, tag="136")]
        CreateForeignServerStmt(super::CreateForeignServerStmt),
        #[prost(message, tag="137")]
        AlterForeignServerStmt(super::AlterForeignServerStmt),
        #[prost(message, tag="138")]
        CreateUserMappingStmt(super::CreateUserMappingStmt),
        #[prost(message, tag="139")]
        AlterUserMappingStmt(super::AlterUserMappingStmt),
        #[prost(message, tag="140")]
        DropUserMappingStmt(super::DropUserMappingStmt),
        #[prost(message, tag="141")]
        AlterTableSpaceOptionsStmt(super::AlterTableSpaceOptionsStmt),
        #[prost(message, tag="142")]
        AlterTableMoveAllStmt(super::AlterTableMoveAllStmt),
        #[prost(message, tag="143")]
        SecLabelStmt(::prost::alloc::boxed::Box<super::SecLabelStmt>),
        #[prost(message, tag="144")]
        CreateForeignTableStmt(super::CreateForeignTableStmt),
        #[prost(message, tag="145")]
        ImportForeignSchemaStmt(super::ImportForeignSchemaStmt),
        #[prost(message, tag="146")]
        CreateExtensionStmt(super::CreateExtensionStmt),
        #[prost(message, tag="147")]
        AlterExtensionStmt(super::AlterExtensionStmt),
        #[prost(message, tag="148")]
        AlterExtensionContentsStmt(::prost::alloc::boxed::Box<super::AlterExtensionContentsStmt>),
        #[prost(message, tag="149")]
        CreateEventTrigStmt(super::CreateEventTrigStmt),
        #[prost(message, tag="150")]
        AlterEventTrigStmt(super::AlterEventTrigStmt),
        #[prost(message, tag="151")]
        RefreshMatViewStmt(super::RefreshMatViewStmt),
        #[prost(message, tag="152")]
        ReplicaIdentityStmt(super::ReplicaIdentityStmt),
        #[prost(message, tag="153")]
        AlterSystemStmt(super::AlterSystemStmt),
        #[prost(message, tag="154")]
        CreatePolicyStmt(::prost::alloc::boxed::Box<super::CreatePolicyStmt>),
        #[prost(message, tag="155")]
        AlterPolicyStmt(::prost::alloc::boxed::Box<super::AlterPolicyStmt>),
        #[prost(message, tag="156")]
        CreateTransformStmt(super::CreateTransformStmt),
        #[prost(message, tag="157")]
        CreateAmStmt(super::CreateAmStmt),
        #[prost(message, tag="158")]
        CreatePublicationStmt(super::CreatePublicationStmt),
        #[prost(message, tag="159")]
        AlterPublicationStmt(super::AlterPublicationStmt),
        #[prost(message, tag="160")]
        CreateSubscriptionStmt(super::CreateSubscriptionStmt),
        #[prost(message, tag="161")]
        AlterSubscriptionStmt(super::AlterSubscriptionStmt),
        #[prost(message, tag="162")]
        DropSubscriptionStmt(super::DropSubscriptionStmt),
        #[prost(message, tag="163")]
        CreateStatsStmt(super::CreateStatsStmt),
        #[prost(message, tag="164")]
        AlterCollationStmt(super::AlterCollationStmt),
        #[prost(message, tag="165")]
        CallStmt(::prost::alloc::boxed::Box<super::CallStmt>),
        #[prost(message, tag="166")]
        AlterStatsStmt(super::AlterStatsStmt),
        #[prost(message, tag="167")]
        AExpr(::prost::alloc::boxed::Box<super::AExpr>),
        #[prost(message, tag="168")]
        ColumnRef(super::ColumnRef),
        #[prost(message, tag="169")]
        ParamRef(super::ParamRef),
        #[prost(message, tag="170")]
        AConst(::prost::alloc::boxed::Box<super::AConst>),
        #[prost(message, tag="171")]
        FuncCall(::prost::alloc::boxed::Box<super::FuncCall>),
        #[prost(message, tag="172")]
        AStar(super::AStar),
        #[prost(message, tag="173")]
        AIndices(::prost::alloc::boxed::Box<super::AIndices>),
        #[prost(message, tag="174")]
        AIndirection(::prost::alloc::boxed::Box<super::AIndirection>),
        #[prost(message, tag="175")]
        AArrayExpr(super::AArrayExpr),
        #[prost(message, tag="176")]
        ResTarget(::prost::alloc::boxed::Box<super::ResTarget>),
        #[prost(message, tag="177")]
        MultiAssignRef(::prost::alloc::boxed::Box<super::MultiAssignRef>),
        #[prost(message, tag="178")]
        TypeCast(::prost::alloc::boxed::Box<super::TypeCast>),
        #[prost(message, tag="179")]
        CollateClause(::prost::alloc::boxed::Box<super::CollateClause>),
        #[prost(message, tag="180")]
        SortBy(::prost::alloc::boxed::Box<super::SortBy>),
        #[prost(message, tag="181")]
        WindowDef(::prost::alloc::boxed::Box<super::WindowDef>),
        #[prost(message, tag="182")]
        RangeSubselect(::prost::alloc::boxed::Box<super::RangeSubselect>),
        #[prost(message, tag="183")]
        RangeFunction(super::RangeFunction),
        #[prost(message, tag="184")]
        RangeTableSample(::prost::alloc::boxed::Box<super::RangeTableSample>),
        #[prost(message, tag="185")]
        RangeTableFunc(::prost::alloc::boxed::Box<super::RangeTableFunc>),
        #[prost(message, tag="186")]
        RangeTableFuncCol(::prost::alloc::boxed::Box<super::RangeTableFuncCol>),
        #[prost(message, tag="187")]
        TypeName(super::TypeName),
        #[prost(message, tag="188")]
        ColumnDef(::prost::alloc::boxed::Box<super::ColumnDef>),
        #[prost(message, tag="189")]
        IndexElem(::prost::alloc::boxed::Box<super::IndexElem>),
        #[prost(message, tag="190")]
        Constraint(::prost::alloc::boxed::Box<super::Constraint>),
        #[prost(message, tag="191")]
        DefElem(::prost::alloc::boxed::Box<super::DefElem>),
        #[prost(message, tag="192")]
        RangeTblEntry(::prost::alloc::boxed::Box<super::RangeTblEntry>),
        #[prost(message, tag="193")]
        RangeTblFunction(::prost::alloc::boxed::Box<super::RangeTblFunction>),
        #[prost(message, tag="194")]
        TableSampleClause(::prost::alloc::boxed::Box<super::TableSampleClause>),
        #[prost(message, tag="195")]
        WithCheckOption(::prost::alloc::boxed::Box<super::WithCheckOption>),
        #[prost(message, tag="196")]
        SortGroupClause(super::SortGroupClause),
        #[prost(message, tag="197")]
        GroupingSet(super::GroupingSet),
        #[prost(message, tag="198")]
        WindowClause(::prost::alloc::boxed::Box<super::WindowClause>),
        #[prost(message, tag="199")]
        ObjectWithArgs(super::ObjectWithArgs),
        #[prost(message, tag="200")]
        AccessPriv(super::AccessPriv),
        #[prost(message, tag="201")]
        CreateOpClassItem(super::CreateOpClassItem),
        #[prost(message, tag="202")]
        TableLikeClause(super::TableLikeClause),
        #[prost(message, tag="203")]
        FunctionParameter(::prost::alloc::boxed::Box<super::FunctionParameter>),
        #[prost(message, tag="204")]
        LockingClause(super::LockingClause),
        #[prost(message, tag="205")]
        RowMarkClause(super::RowMarkClause),
        #[prost(message, tag="206")]
        XmlSerialize(::prost::alloc::boxed::Box<super::XmlSerialize>),
        #[prost(message, tag="207")]
        WithClause(super::WithClause),
        #[prost(message, tag="208")]
        InferClause(::prost::alloc::boxed::Box<super::InferClause>),
        #[prost(message, tag="209")]
        OnConflictClause(::prost::alloc::boxed::Box<super::OnConflictClause>),
        #[prost(message, tag="210")]
        CommonTableExpr(::prost::alloc::boxed::Box<super::CommonTableExpr>),
        #[prost(message, tag="211")]
        RoleSpec(super::RoleSpec),
        #[prost(message, tag="212")]
        TriggerTransition(super::TriggerTransition),
        #[prost(message, tag="213")]
        PartitionElem(::prost::alloc::boxed::Box<super::PartitionElem>),
        #[prost(message, tag="214")]
        PartitionSpec(super::PartitionSpec),
        #[prost(message, tag="215")]
        PartitionBoundSpec(super::PartitionBoundSpec),
        #[prost(message, tag="216")]
        PartitionRangeDatum(::prost::alloc::boxed::Box<super::PartitionRangeDatum>),
        #[prost(message, tag="217")]
        PartitionCmd(super::PartitionCmd),
        #[prost(message, tag="218")]
        VacuumRelation(super::VacuumRelation),
        #[prost(message, tag="219")]
        InlineCodeBlock(super::InlineCodeBlock),
        #[prost(message, tag="220")]
        CallContext(super::CallContext),
        #[prost(message, tag="221")]
        Integer(super::Integer),
        #[prost(message, tag="222")]
        Float(super::Float),
        #[prost(message, tag="223")]
        String(super::String),
        #[prost(message, tag="224")]
        BitString(super::BitString),
        #[prost(message, tag="225")]
        Null(super::Null),
        #[prost(message, tag="226")]
        List(super::List),
        #[prost(message, tag="227")]
        IntList(super::IntList),
        #[prost(message, tag="228")]
        OidList(super::OidList),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Integer {
    /// machine integer 
    #[prost(int32, tag="1")]
    pub ival: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Float {
    /// string 
    #[prost(string, tag="1")]
    pub str: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct String {
    /// string 
    #[prost(string, tag="1")]
    pub str: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BitString {
    /// string 
    #[prost(string, tag="1")]
    pub str: ::prost::alloc::string::String,
}
/// intentionally empty
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Null {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct List {
    #[prost(message, repeated, tag="1")]
    pub items: ::prost::alloc::vec::Vec<Node>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OidList {
    #[prost(message, repeated, tag="1")]
    pub items: ::prost::alloc::vec::Vec<Node>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IntList {
    #[prost(message, repeated, tag="1")]
    pub items: ::prost::alloc::vec::Vec<Node>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Alias {
    #[prost(string, tag="1")]
    pub aliasname: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="2")]
    pub colnames: ::prost::alloc::vec::Vec<Node>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RangeVar {
    #[prost(string, tag="1")]
    pub catalogname: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub schemaname: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub relname: ::prost::alloc::string::String,
    #[prost(bool, tag="4")]
    pub inh: bool,
    #[prost(string, tag="5")]
    pub relpersistence: ::prost::alloc::string::String,
    #[prost(message, optional, tag="6")]
    pub alias: ::core::option::Option<Alias>,
    #[prost(int32, tag="7")]
    pub location: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TableFunc {
    #[prost(message, repeated, tag="1")]
    pub ns_uris: ::prost::alloc::vec::Vec<Node>,
    #[prost(message, repeated, tag="2")]
    pub ns_names: ::prost::alloc::vec::Vec<Node>,
    #[prost(message, optional, boxed, tag="3")]
    pub docexpr: ::core::option::Option<::prost::alloc::boxed::Box<Node>>,
    #[prost(message, optional, boxed, tag="4")]
    pub rowexpr: ::core::option::Option<::prost::alloc::boxed::Box<Node>>,
    #[prost(message, repeated, tag="5")]
    pub colnames: ::prost::alloc::vec::Vec<Node>,
    #[prost(message, repeated, tag="6")]
    pub coltypes: ::prost::alloc::vec::Vec<Node>,
    #[prost(message, repeated, tag="7")]
    pub coltypmods: ::prost::alloc::vec::Vec<Node>,
    #[prost(message, repeated, tag="8")]
    pub colcollations: ::prost::alloc::vec::Vec<Node>,
    #[prost(message, repeated, tag="9")]
    pub colexprs: ::prost::alloc::vec::Vec<Node>,
    #[prost(message, repeated, tag="10")]
    pub coldefexprs: ::prost::alloc::vec::Vec<Node>,
    #[prost(uint64, repeated, tag="11")]
    pub notnulls: ::prost::alloc::vec::Vec<u64>,
    #[prost(int32, tag="12")]
    pub ordinalitycol: i32,
    #[prost(int32, tag="13")]
    pub location: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Expr {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Var {
    #[prost(message, optional, boxed, tag="1")]
    pub xpr: ::core::option::Option<::prost::alloc::boxed::Box<Node>>,
    #[prost(uint32, tag="2")]
    pub varno: u32,
    #[prost(int32, tag="3")]
    pub varattno: i32,
    #[prost(uint32, tag="4")]
    pub vartype: u32,
    #[prost(int32, tag="5")]
    pub vartypmod: i32,
    #[prost(uint32, tag="6")]
    pub varcollid: u32,
    #[prost(uint32, tag="7")]
    pub varlevelsup: u32,
    #[prost(uint32, tag="8")]
    pub varnosyn: u32,
    #[prost(int32, tag="9")]
    pub varattnosyn: i32,
    #[prost(int32, tag="10")]
    pub location: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Param {
    #[prost(message, optional, boxed, tag="1")]
    pub xpr: ::core::option::Option<::prost::alloc::boxed::Box<Node>>,
    #[prost(enumeration="ParamKind", tag="2")]
    pub paramkind: i32,
    #[prost(int32, tag="3")]
    pub paramid: i32,
    #[prost(uint32, tag="4")]
    pub paramtype: u32,
    #[prost(int32, tag="5")]
    pub paramtypmod: i32,
    #[prost(uint32, tag="6")]
    pub paramcollid: u32,
    #[prost(int32, tag="7")]
    pub location: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Aggref {
    #[prost(message, optional, boxed, tag="1")]
    pub xpr: ::core::option::Option<::prost::alloc::boxed::Box<Node>>,
    #[prost(uint32, tag="2")]
    pub aggfnoid: u32,
    #[prost(uint32, tag="3")]
    pub aggtype: u32,
    #[prost(uint32, tag="4")]
    pub aggcollid: u32,
    #[prost(uint32, tag="5")]
    pub inputcollid: u32,
    #[prost(uint32, tag="6")]
    pub aggtranstype: u32,
    #[prost(message, repeated, tag="7")]
    pub aggargtypes: ::prost::alloc::vec::Vec<Node>,
    #[prost(message, repeated, tag="8")]
    pub aggdirectargs: ::prost::alloc::vec::Vec<Node>,
    #[prost(message, repeated, tag="9")]
    pub args: ::prost::alloc::vec::Vec<Node>,
    #[prost(message, repeated, tag="10")]
    pub aggorder: ::prost::alloc::vec::Vec<Node>,
    #[prost(message, repeated, tag="11")]
    pub aggdistinct: ::prost::alloc::vec::Vec<Node>,
    #[prost(message, optional, boxed, tag="12")]
    pub aggfilter: ::core::option::Option<::prost::alloc::boxed::Box<Node>>,
    #[prost(bool, tag="13")]
    pub aggstar: bool,
    #[prost(bool, tag="14")]
    pub aggvariadic: bool,
    #[prost(string, tag="15")]
    pub aggkind: ::prost::alloc::string::String,
    #[prost(uint32, tag="16")]
    pub agglevelsup: u32,
    #[prost(enumeration="AggSplit", tag="17")]
    pub aggsplit: i32,
    #[prost(int32, tag="18")]
    pub location: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupingFunc {
    #[prost(message, optional, boxed, tag="1")]
    pub xpr: ::core::option::Option<::prost::alloc::boxed::Box<Node>>,
    #[prost(message, repeated, tag="2")]
    pub args: ::prost::alloc::vec::Vec<Node>,
    #[prost(message, repeated, tag="3")]
    pub refs: ::prost::alloc::vec::Vec<Node>,
    #[prost(message, repeated, tag="4")]
    pub cols: ::prost::alloc::vec::Vec<Node>,
    #[prost(uint32, tag="5")]
    pub agglevelsup: u32,
    #[prost(int32, tag="6")]
    pub location: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WindowFunc {
    #[prost(message, optional, boxed, tag="1")]
    pub xpr: ::core::option::Option<::prost::alloc::boxed::Box<Node>>,
    #[prost(uint32, tag="2")]
    pub winfnoid: u32,
    #[prost(uint32, tag="3")]
    pub wintype: u32,
    #[prost(uint32, tag="4")]
    pub wincollid: u32,
    #[prost(uint32, tag="5")]
    pub inputcollid: u32,
    #[prost(message, repeated, tag="6")]
    pub args: ::prost::alloc::vec::Vec<Node>,
    #[prost(message, optional, boxed, tag="7")]
    pub aggfilter: ::core::option::Option<::prost::alloc::boxed::Box<Node>>,
    #[prost(uint32, tag="8")]
    pub winref: u32,
    #[prost(bool, tag="9")]
    pub winstar: bool,
    #[prost(bool, tag="10")]
    pub winagg: bool,
    #[prost(int32, tag="11")]
    pub location: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscriptingRef {
    #[prost(message, optional, boxed, tag="1")]
    pub xpr: ::core::option::Option<::prost::alloc::boxed::Box<Node>>,
    #[prost(uint32, tag="2")]
    pub refcontainertype: u32,
    #[prost(uint32, tag="3")]
    pub refelemtype: u32,
    #[prost(int32, tag="4")]
    pub reftypmod: i32,
    #[prost(uint32, tag="5")]
    pub refcollid: u32,
    #[prost(message, repeated, tag="6")]
    pub refupperindexpr: ::prost::alloc::vec::Vec<Node>,
    #[prost(message, repeated, tag="7")]
    pub reflowerindexpr: ::prost::alloc::vec::Vec<Node>,
    #[prost(message, optional, boxed, tag="8")]
    pub refexpr: ::core::option::Option<::prost::alloc::boxed::Box<Node>>,
    #[prost(message, optional, boxed, tag="9")]
    pub refassgnexpr: ::core::option::Option<::prost::alloc::boxed::Box<Node>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FuncExpr {
    #[prost(message, optional, boxed, tag="1")]
    pub xpr: ::core::option::Option<::prost::alloc::boxed::Box<Node>>,
    #[prost(uint32, tag="2")]
    pub funcid: u32,
    #[prost(uint32, tag="3")]
    pub funcresulttype: u32,
    #[prost(bool, tag="4")]
    pub funcretset: bool,
    #[prost(bool, tag="5")]
    pub funcvariadic: bool,
    #[prost(enumeration="CoercionForm", tag="6")]
    pub funcformat: i32,
    #[prost(uint32, tag="7")]
    pub funccollid: u32,
    #[prost(uint32, tag="8")]
    pub inputcollid: u32,
    #[prost(message, repeated, tag="9")]
    pub args: ::prost::alloc::vec::Vec<Node>,
    #[prost(int32, tag="10")]
    pub location: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NamedArgExpr {
    #[prost(message, optional, boxed, tag="1")]
    pub xpr: ::core::option::Option<::prost::alloc::boxed::Box<Node>>,
    #[prost(message, optional, boxed, tag="2")]
    pub arg: ::core::option::Option<::prost::alloc::boxed::Box<Node>>,
    #[prost(string, tag="3")]
    pub name: ::prost::alloc::string::String,
    #[prost(int32, tag="4")]
    pub argnumber: i32,
    #[prost(int32, tag="5")]
    pub location: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpExpr {
    #[prost(message, optional, boxed, tag="1")]
    pub xpr: ::core::option::Option<::prost::alloc::boxed::Box<Node>>,
    #[prost(uint32, tag="2")]
    pub opno: u32,
    #[prost(uint32, tag="3")]
    pub opfuncid: u32,
    #[prost(uint32, tag="4")]
    pub opresulttype: u32,
    #[prost(bool, tag="5")]
    pub opretset: bool,
    #[prost(uint32, tag="6")]
    pub opcollid: u32,
    #[prost(uint32, tag="7")]
    pub inputcollid: u32,
    #[prost(message, repeated, tag="8")]
    pub args: ::prost::alloc::vec::Vec<Node>,
    #[prost(int32, tag="9")]
    pub location: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DistinctExpr {
    #[prost(message, optional, boxed, tag="1")]
    pub xpr: ::core::option::Option<::prost::alloc::boxed::Box<Node>>,
    #[prost(uint32, tag="2")]
    pub opno: u32,
    #[prost(uint32, tag="3")]
    pub opfuncid: u32,
    #[prost(uint32, tag="4")]
    pub opresulttype: u32,
    #[prost(bool, tag="5")]
    pub opretset: bool,
    #[prost(uint32, tag="6")]
    pub opcollid: u32,
    #[prost(uint32, tag="7")]
    pub inputcollid: u32,
    #[prost(message, repeated, tag="8")]
    pub args: ::prost::alloc::vec::Vec<Node>,
    #[prost(int32, tag="9")]
    pub location: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NullIfExpr {
    #[prost(message, optional, boxed, tag="1")]
    pub xpr: ::core::option::Option<::prost::alloc::boxed::Box<Node>>,
    #[prost(uint32, tag="2")]
    pub opno: u32,
    #[prost(uint32, tag="3")]
    pub opfuncid: u32,
    #[prost(uint32, tag="4")]
    pub opresulttype: u32,
    #[prost(bool, tag="5")]
    pub opretset: bool,
    #[prost(uint32, tag="6")]
    pub opcollid: u32,
    #[prost(uint32, tag="7")]
    pub inputcollid: u32,
    #[prost(message, repeated, tag="8")]
    pub args: ::prost::alloc::vec::Vec<Node>,
    #[prost(int32, tag="9")]
    pub location: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScalarArrayOpExpr {
    #[prost(message, optional, boxed, tag="1")]
    pub xpr: ::core::option::Option<::prost::alloc::boxed::Box<Node>>,
    #[prost(uint32, tag="2")]
    pub opno: u32,
    #[prost(uint32, tag="3")]
    pub opfuncid: u32,
    #[prost(bool, tag="4")]
    pub use_or: bool,
    #[prost(uint32, tag="5")]
    pub inputcollid: u32,
    #[prost(message, repeated, tag="6")]
    pub args: ::prost::alloc::vec::Vec<Node>,
    #[prost(int32, tag="7")]
    pub location: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BoolExpr {
    #[prost(message, optional, boxed, tag="1")]
    pub xpr: ::core::option::Option<::prost::alloc::boxed::Box<Node>>,
    #[prost(enumeration="BoolExprType", tag="2")]
    pub boolop: i32,
    #[prost(message, repeated, tag="3")]
    pub args: ::prost::alloc::vec::Vec<Node>,
    #[prost(int32, tag="4")]
    pub location: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubLink {
    #[prost(message, optional, boxed, tag="1")]
    pub xpr: ::core::option::Option<::prost::alloc::boxed::Box<Node>>,
    #[prost(enumeration="SubLinkType", tag="2")]
    pub sub_link_type: i32,
    #[prost(int32, tag="3")]
    pub sub_link_id: i32,
    #[prost(message, optional, boxed, tag="4")]
    pub testexpr: ::core::option::Option<::prost::alloc::boxed::Box<Node>>,
    #[prost(message, repeated, tag="5")]
    pub oper_name: ::prost::alloc::vec::Vec<Node>,
    #[prost(message, optional, boxed, tag="6")]
    pub subselect: ::core::option::Option<::prost::alloc::boxed::Box<Node>>,
    #[prost(int32, tag="7")]
    pub location: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubPlan {
    #[prost(message, optional, boxed, tag="1")]
    pub xpr: ::core::option::Option<::prost::alloc::boxed::Box<Node>>,
    #[prost(enumeration="SubLinkType", tag="2")]
    pub sub_link_type: i32,
    #[prost(message, optional, boxed, tag="3")]
    pub testexpr: ::core::option::Option<::prost::alloc::boxed::Box<Node>>,
    #[prost(message, repeated, tag="4")]
    pub param_ids: ::prost::alloc::vec::Vec<Node>,
    #[prost(int32, tag="5")]
    pub plan_id: i32,
    #[prost(string, tag="6")]
    pub plan_name: ::prost::alloc::string::String,
    #[prost(uint32, tag="7")]
    pub first_col_type: u32,
    #[prost(int32, tag="8")]
    pub first_col_typmod: i32,
    #[prost(uint32, tag="9")]
    pub first_col_collation: u32,
    #[prost(bool, tag="10")]
    pub use_hash_table: bool,
    #[prost(bool, tag="11")]
    pub unknown_eq_false: bool,
    #[prost(bool, tag="12")]
    pub parallel_safe: bool,
    #[prost(message, repeated, tag="13")]
    pub set_param: ::prost::alloc::vec::Vec<Node>,
    #[prost(message, repeated, tag="14")]
    pub par_param: ::prost::alloc::vec::Vec<Node>,
    #[prost(message, repeated, tag="15")]
    pub args: ::prost::alloc::vec::Vec<Node>,
    #[prost(double, tag="16")]
    pub startup_cost: f64,
    #[prost(double, tag="17")]
    pub per_call_cost: f64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AlternativeSubPlan {
    #[prost(message, optional, boxed, tag="1")]
    pub xpr: ::core::option::Option<::prost::alloc::boxed::Box<Node>>,
    #[prost(message, repeated, tag="2")]
    pub subplans: ::prost::alloc::vec::Vec<Node>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FieldSelect {
    #[prost(message, optional, boxed, tag="1")]
    pub xpr: ::core::option::Option<::prost::alloc::boxed::Box<Node>>,
    #[prost(message, optional, boxed, tag="2")]
    pub arg: ::core::option::Option<::prost::alloc::boxed::Box<Node>>,
    #[prost(int32, tag="3")]
    pub fieldnum: i32,
    #[prost(uint32, tag="4")]
    pub resulttype: u32,
    #[prost(int32, tag="5")]
    pub resulttypmod: i32,
    #[prost(uint32, tag="6")]
    pub resultcollid: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FieldStore {
    #[prost(message, optional, boxed, tag="1")]
    pub xpr: ::core::option::Option<::prost::alloc::boxed::Box<Node>>,
    #[prost(message, optional, boxed, tag="2")]
    pub arg: ::core::option::Option<::prost::alloc::boxed::Box<Node>>,
    #[prost(message, repeated, tag="3")]
    pub newvals: ::prost::alloc::vec::Vec<Node>,
    #[prost(message, repeated, tag="4")]
    pub fieldnums: ::prost::alloc::vec::Vec<Node>,
    #[prost(uint32, tag="5")]
    pub resulttype: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RelabelType {
    #[prost(message, optional, boxed, tag="1")]
    pub xpr: ::core::option::Option<::prost::alloc::boxed::Box<Node>>,
    #[prost(message, optional, boxed, tag="2")]
    pub arg: ::core::option::Option<::prost::alloc::boxed::Box<Node>>,
    #[prost(uint32, tag="3")]
    pub resulttype: u32,
    #[prost(int32, tag="4")]
    pub resulttypmod: i32,
    #[prost(uint32, tag="5")]
    pub resultcollid: u32,
    #[prost(enumeration="CoercionForm", tag="6")]
    pub relabelformat: i32,
    #[prost(int32, tag="7")]
    pub location: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CoerceViaIo {
    #[prost(message, optional, boxed, tag="1")]
    pub xpr: ::core::option::Option<::prost::alloc::boxed::Box<Node>>,
    #[prost(message, optional, boxed, tag="2")]
    pub arg: ::core::option::Option<::prost::alloc::boxed::Box<Node>>,
    #[prost(uint32, tag="3")]
    pub resulttype: u32,
    #[prost(uint32, tag="4")]
    pub resultcollid: u32,
    #[prost(enumeration="CoercionForm", tag="5")]
    pub coerceformat: i32,
    #[prost(int32, tag="6")]
    pub location: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ArrayCoerceExpr {
    #[prost(message, optional, boxed, tag="1")]
    pub xpr: ::core::option::Option<::prost::alloc::boxed::Box<Node>>,
    #[prost(message, optional, boxed, tag="2")]
    pub arg: ::core::option::Option<::prost::alloc::boxed::Box<Node>>,
    #[prost(message, optional, boxed, tag="3")]
    pub elemexpr: ::core::option::Option<::prost::alloc::boxed::Box<Node>>,
    #[prost(uint32, tag="4")]
    pub resulttype: u32,
    #[prost(int32, tag="5")]
    pub resulttypmod: i32,
    #[prost(uint32, tag="6")]
    pub resultcollid: u32,
    #[prost(enumeration="CoercionForm", tag="7")]
    pub coerceformat: i32,
    #[prost(int32, tag="8")]
    pub location: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConvertRowtypeExpr {
    #[prost(message, optional, boxed, tag="1")]
    pub xpr: ::core::option::Option<::prost::alloc::boxed::Box<Node>>,
    #[prost(message, optional, boxed, tag="2")]
    pub arg: ::core::option::Option<::prost::alloc::boxed::Box<Node>>,
    #[prost(uint32, tag="3")]
    pub resulttype: u32,
    #[prost(enumeration="CoercionForm", tag="4")]
    pub convertformat: i32,
    #[prost(int32, tag="5")]
    pub location: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CollateExpr {
    #[prost(message, optional, boxed, tag="1")]
    pub xpr: ::core::option::Option<::prost::alloc::boxed::Box<Node>>,
    #[prost(message, optional, boxed, tag="2")]
    pub arg: ::core::option::Option<::prost::alloc::boxed::Box<Node>>,
    #[prost(uint32, tag="3")]
    pub coll_oid: u32,
    #[prost(int32, tag="4")]
    pub location: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CaseExpr {
    #[prost(message, optional, boxed, tag="1")]
    pub xpr: ::core::option::Option<::prost::alloc::boxed::Box<Node>>,
    #[prost(uint32, tag="2")]
    pub casetype: u32,
    #[prost(uint32, tag="3")]
    pub casecollid: u32,
    #[prost(message, optional, boxed, tag="4")]
    pub arg: ::core::option::Option<::prost::alloc::boxed::Box<Node>>,
    #[prost(message, repeated, tag="5")]
    pub args: ::prost::alloc::vec::Vec<Node>,
    #[prost(message, optional, boxed, tag="6")]
    pub defresult: ::core::option::Option<::prost::alloc::boxed::Box<Node>>,
    #[prost(int32, tag="7")]
    pub location: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CaseWhen {
    #[prost(message, optional, boxed, tag="1")]
    pub xpr: ::core::option::Option<::prost::alloc::boxed::Box<Node>>,
    #[prost(message, optional, boxed, tag="2")]
    pub expr: ::core::option::Option<::prost::alloc::boxed::Box<Node>>,
    #[prost(message, optional, boxed, tag="3")]
    pub result: ::core::option::Option<::prost::alloc::boxed::Box<Node>>,
    #[prost(int32, tag="4")]
    pub location: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CaseTestExpr {
    #[prost(message, optional, boxed, tag="1")]
    pub xpr: ::core::option::Option<::prost::alloc::boxed::Box<Node>>,
    #[prost(uint32, tag="2")]
    pub type_id: u32,
    #[prost(int32, tag="3")]
    pub type_mod: i32,
    #[prost(uint32, tag="4")]
    pub collation: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ArrayExpr {
    #[prost(message, optional, boxed, tag="1")]
    pub xpr: ::core::option::Option<::prost::alloc::boxed::Box<Node>>,
    #[prost(uint32, tag="2")]
    pub array_typeid: u32,
    #[prost(uint32, tag="3")]
    pub array_collid: u32,
    #[prost(uint32, tag="4")]
    pub element_typeid: u32,
    #[prost(message, repeated, tag="5")]
    pub elements: ::prost::alloc::vec::Vec<Node>,
    #[prost(bool, tag="6")]
    pub multidims: bool,
    #[prost(int32, tag="7")]
    pub location: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RowExpr {
    #[prost(message, optional, boxed, tag="1")]
    pub xpr: ::core::option::Option<::prost::alloc::boxed::Box<Node>>,
    #[prost(message, repeated, tag="2")]
    pub args: ::prost::alloc::vec::Vec<Node>,
    #[prost(uint32, tag="3")]
    pub row_typeid: u32,
    #[prost(enumeration="CoercionForm", tag="4")]
    pub row_format: i32,
    #[prost(message, repeated, tag="5")]
    pub colnames: ::prost::alloc::vec::Vec<Node>,
    #[prost(int32, tag="6")]
    pub location: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RowCompareExpr {
    #[prost(message, optional, boxed, tag="1")]
    pub xpr: ::core::option::Option<::prost::alloc::boxed::Box<Node>>,
    #[prost(enumeration="RowCompareType", tag="2")]
    pub rctype: i32,
    #[prost(message, repeated, tag="3")]
    pub opnos: ::prost::alloc::vec::Vec<Node>,
    #[prost(message, repeated, tag="4")]
    pub opfamilies: ::prost::alloc::vec::Vec<Node>,
    #[prost(message, repeated, tag="5")]
    pub inputcollids: ::prost::alloc::vec::Vec<Node>,
    #[prost(message, repeated, tag="6")]
    pub largs: ::prost::alloc::vec::Vec<Node>,
    #[prost(message, repeated, tag="7")]
    pub rargs: ::prost::alloc::vec::Vec<Node>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CoalesceExpr {
    #[prost(message, optional, boxed, tag="1")]
    pub xpr: ::core::option::Option<::prost::alloc::boxed::Box<Node>>,
    #[prost(uint32, tag="2")]
    pub coalescetype: u32,
    #[prost(uint32, tag="3")]
    pub coalescecollid: u32,
    #[prost(message, repeated, tag="4")]
    pub args: ::prost::alloc::vec::Vec<Node>,
    #[prost(int32, tag="5")]
    pub location: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MinMaxExpr {
    #[prost(message, optional, boxed, tag="1")]
    pub xpr: ::core::option::Option<::prost::alloc::boxed::Box<Node>>,
    #[prost(uint32, tag="2")]
    pub minmaxtype: u32,
    #[prost(uint32, tag="3")]
    pub minmaxcollid: u32,
    #[prost(uint32, tag="4")]
    pub inputcollid: u32,
    #[prost(enumeration="MinMaxOp", tag="5")]
    pub op: i32,
    #[prost(message, repeated, tag="6")]
    pub args: ::prost::alloc::vec::Vec<Node>,
    #[prost(int32, tag="7")]
    pub location: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SqlValueFunction {
    #[prost(message, optional, boxed, tag="1")]
    pub xpr: ::core::option::Option<::prost::alloc::boxed::Box<Node>>,
    #[prost(enumeration="SqlValueFunctionOp", tag="2")]
    pub op: i32,
    #[prost(uint32, tag="3")]
    pub r#type: u32,
    #[prost(int32, tag="4")]
    pub typmod: i32,
    #[prost(int32, tag="5")]
    pub location: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct XmlExpr {
    #[prost(message, optional, boxed, tag="1")]
    pub xpr: ::core::option::Option<::prost::alloc::boxed::Box<Node>>,
    #[prost(enumeration="XmlExprOp", tag="2")]
    pub op: i32,
    #[prost(string, tag="3")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="4")]
    pub named_args: ::prost::alloc::vec::Vec<Node>,
    #[prost(message, repeated, tag="5")]
    pub arg_names: ::prost::alloc::vec::Vec<Node>,
    #[prost(message, repeated, tag="6")]
    pub args: ::prost::alloc::vec::Vec<Node>,
    #[prost(enumeration="XmlOptionType", tag="7")]
    pub xmloption: i32,
    #[prost(uint32, tag="8")]
    pub r#type: u32,
    #[prost(int32, tag="9")]
    pub typmod: i32,
    #[prost(int32, tag="10")]
    pub location: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NullTest {
    #[prost(message, optional, boxed, tag="1")]
    pub xpr: ::core::option::Option<::prost::alloc::boxed::Box<Node>>,
    #[prost(message, optional, boxed, tag="2")]
    pub arg: ::core::option::Option<::prost::alloc::boxed::Box<Node>>,
    #[prost(enumeration="NullTestType", tag="3")]
    pub nulltesttype: i32,
    #[prost(bool, tag="4")]
    pub argisrow: bool,
    #[prost(int32, tag="5")]
    pub location: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BooleanTest {
    #[prost(message, optional, boxed, tag="1")]
    pub xpr: ::core::option::Option<::prost::alloc::boxed::Box<Node>>,
    #[prost(message, optional, boxed, tag="2")]
    pub arg: ::core::option::Option<::prost::alloc::boxed::Box<Node>>,
    #[prost(enumeration="BoolTestType", tag="3")]
    pub booltesttype: i32,
    #[prost(int32, tag="4")]
    pub location: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CoerceToDomain {
    #[prost(message, optional, boxed, tag="1")]
    pub xpr: ::core::option::Option<::prost::alloc::boxed::Box<Node>>,
    #[prost(message, optional, boxed, tag="2")]
    pub arg: ::core::option::Option<::prost::alloc::boxed::Box<Node>>,
    #[prost(uint32, tag="3")]
    pub resulttype: u32,
    #[prost(int32, tag="4")]
    pub resulttypmod: i32,
    #[prost(uint32, tag="5")]
    pub resultcollid: u32,
    #[prost(enumeration="CoercionForm", tag="6")]
    pub coercionformat: i32,
    #[prost(int32, tag="7")]
    pub location: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CoerceToDomainValue {
    #[prost(message, optional, boxed, tag="1")]
    pub xpr: ::core::option::Option<::prost::alloc::boxed::Box<Node>>,
    #[prost(uint32, tag="2")]
    pub type_id: u32,
    #[prost(int32, tag="3")]
    pub type_mod: i32,
    #[prost(uint32, tag="4")]
    pub collation: u32,
    #[prost(int32, tag="5")]
    pub location: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetToDefault {
    #[prost(message, optional, boxed, tag="1")]
    pub xpr: ::core::option::Option<::prost::alloc::boxed::Box<Node>>,
    #[prost(uint32, tag="2")]
    pub type_id: u32,
    #[prost(int32, tag="3")]
    pub type_mod: i32,
    #[prost(uint32, tag="4")]
    pub collation: u32,
    #[prost(int32, tag="5")]
    pub location: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CurrentOfExpr {
    #[prost(message, optional, boxed, tag="1")]
    pub xpr: ::core::option::Option<::prost::alloc::boxed::Box<Node>>,
    #[prost(uint32, tag="2")]
    pub cvarno: u32,
    #[prost(string, tag="3")]
    pub cursor_name: ::prost::alloc::string::String,
    #[prost(int32, tag="4")]
    pub cursor_param: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NextValueExpr {
    #[prost(message, optional, boxed, tag="1")]
    pub xpr: ::core::option::Option<::prost::alloc::boxed::Box<Node>>,
    #[prost(uint32, tag="2")]
    pub seqid: u32,
    #[prost(uint32, tag="3")]
    pub type_id: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InferenceElem {
    #[prost(message, optional, boxed, tag="1")]
    pub xpr: ::core::option::Option<::prost::alloc::boxed::Box<Node>>,
    #[prost(message, optional, boxed, tag="2")]
    pub expr: ::core::option::Option<::prost::alloc::boxed::Box<Node>>,
    #[prost(uint32, tag="3")]
    pub infercollid: u32,
    #[prost(uint32, tag="4")]
    pub inferopclass: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TargetEntry {
    #[prost(message, optional, boxed, tag="1")]
    pub xpr: ::core::option::Option<::prost::alloc::boxed::Box<Node>>,
    #[prost(message, optional, boxed, tag="2")]
    pub expr: ::core::option::Option<::prost::alloc::boxed::Box<Node>>,
    #[prost(int32, tag="3")]
    pub resno: i32,
    #[prost(string, tag="4")]
    pub resname: ::prost::alloc::string::String,
    #[prost(uint32, tag="5")]
    pub ressortgroupref: u32,
    #[prost(uint32, tag="6")]
    pub resorigtbl: u32,
    #[prost(int32, tag="7")]
    pub resorigcol: i32,
    #[prost(bool, tag="8")]
    pub resjunk: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RangeTblRef {
    #[prost(int32, tag="1")]
    pub rtindex: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct JoinExpr {
    #[prost(enumeration="JoinType", tag="1")]
    pub jointype: i32,
    #[prost(bool, tag="2")]
    pub is_natural: bool,
    #[prost(message, optional, boxed, tag="3")]
    pub larg: ::core::option::Option<::prost::alloc::boxed::Box<Node>>,
    #[prost(message, optional, boxed, tag="4")]
    pub rarg: ::core::option::Option<::prost::alloc::boxed::Box<Node>>,
    #[prost(message, repeated, tag="5")]
    pub using_clause: ::prost::alloc::vec::Vec<Node>,
    #[prost(message, optional, boxed, tag="6")]
    pub quals: ::core::option::Option<::prost::alloc::boxed::Box<Node>>,
    #[prost(message, optional, tag="7")]
    pub alias: ::core::option::Option<Alias>,
    #[prost(int32, tag="8")]
    pub rtindex: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FromExpr {
    #[prost(message, repeated, tag="1")]
    pub fromlist: ::prost::alloc::vec::Vec<Node>,
    #[prost(message, optional, boxed, tag="2")]
    pub quals: ::core::option::Option<::prost::alloc::boxed::Box<Node>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OnConflictExpr {
    #[prost(enumeration="OnConflictAction", tag="1")]
    pub action: i32,
    #[prost(message, repeated, tag="2")]
    pub arbiter_elems: ::prost::alloc::vec::Vec<Node>,
    #[prost(message, optional, boxed, tag="3")]
    pub arbiter_where: ::core::option::Option<::prost::alloc::boxed::Box<Node>>,
    #[prost(uint32, tag="4")]
    pub constraint: u32,
    #[prost(message, repeated, tag="5")]
    pub on_conflict_set: ::prost::alloc::vec::Vec<Node>,
    #[prost(message, optional, boxed, tag="6")]
    pub on_conflict_where: ::core::option::Option<::prost::alloc::boxed::Box<Node>>,
    #[prost(int32, tag="7")]
    pub excl_rel_index: i32,
    #[prost(message, repeated, tag="8")]
    pub excl_rel_tlist: ::prost::alloc::vec::Vec<Node>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IntoClause {
    #[prost(message, optional, tag="1")]
    pub rel: ::core::option::Option<RangeVar>,
    #[prost(message, repeated, tag="2")]
    pub col_names: ::prost::alloc::vec::Vec<Node>,
    #[prost(string, tag="3")]
    pub access_method: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="4")]
    pub options: ::prost::alloc::vec::Vec<Node>,
    #[prost(enumeration="OnCommitAction", tag="5")]
    pub on_commit: i32,
    #[prost(string, tag="6")]
    pub table_space_name: ::prost::alloc::string::String,
    #[prost(message, optional, boxed, tag="7")]
    pub view_query: ::core::option::Option<::prost::alloc::boxed::Box<Node>>,
    #[prost(bool, tag="8")]
    pub skip_data: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RawStmt {
    #[prost(message, optional, boxed, tag="1")]
    pub stmt: ::core::option::Option<::prost::alloc::boxed::Box<Node>>,
    #[prost(int32, tag="2")]
    pub stmt_location: i32,
    #[prost(int32, tag="3")]
    pub stmt_len: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Query {
    #[prost(enumeration="CmdType", tag="1")]
    pub command_type: i32,
    #[prost(enumeration="QuerySource", tag="2")]
    pub query_source: i32,
    #[prost(bool, tag="3")]
    pub can_set_tag: bool,
    #[prost(message, optional, boxed, tag="4")]
    pub utility_stmt: ::core::option::Option<::prost::alloc::boxed::Box<Node>>,
    #[prost(int32, tag="5")]
    pub result_relation: i32,
    #[prost(bool, tag="6")]
    pub has_aggs: bool,
    #[prost(bool, tag="7")]
    pub has_window_funcs: bool,
    #[prost(bool, tag="8")]
    pub has_target_srfs: bool,
    #[prost(bool, tag="9")]
    pub has_sub_links: bool,
    #[prost(bool, tag="10")]
    pub has_distinct_on: bool,
    #[prost(bool, tag="11")]
    pub has_recursive: bool,
    #[prost(bool, tag="12")]
    pub has_modifying_cte: bool,
    #[prost(bool, tag="13")]
    pub has_for_update: bool,
    #[prost(bool, tag="14")]
    pub has_row_security: bool,
    #[prost(message, repeated, tag="15")]
    pub cte_list: ::prost::alloc::vec::Vec<Node>,
    #[prost(message, repeated, tag="16")]
    pub rtable: ::prost::alloc::vec::Vec<Node>,
    #[prost(message, optional, boxed, tag="17")]
    pub jointree: ::core::option::Option<::prost::alloc::boxed::Box<FromExpr>>,
    #[prost(message, repeated, tag="18")]
    pub target_list: ::prost::alloc::vec::Vec<Node>,
    #[prost(enumeration="OverridingKind", tag="19")]
    pub r#override: i32,
    #[prost(message, optional, boxed, tag="20")]
    pub on_conflict: ::core::option::Option<::prost::alloc::boxed::Box<OnConflictExpr>>,
    #[prost(message, repeated, tag="21")]
    pub returning_list: ::prost::alloc::vec::Vec<Node>,
    #[prost(message, repeated, tag="22")]
    pub group_clause: ::prost::alloc::vec::Vec<Node>,
    #[prost(message, repeated, tag="23")]
    pub grouping_sets: ::prost::alloc::vec::Vec<Node>,
    #[prost(message, optional, boxed, tag="24")]
    pub having_qual: ::core::option::Option<::prost::alloc::boxed::Box<Node>>,
    #[prost(message, repeated, tag="25")]
    pub window_clause: ::prost::alloc::vec::Vec<Node>,
    #[prost(message, repeated, tag="26")]
    pub distinct_clause: ::prost::alloc::vec::Vec<Node>,
    #[prost(message, repeated, tag="27")]
    pub sort_clause: ::prost::alloc::vec::Vec<Node>,
    #[prost(message, optional, boxed, tag="28")]
    pub limit_offset: ::core::option::Option<::prost::alloc::boxed::Box<Node>>,
    #[prost(message, optional, boxed, tag="29")]
    pub limit_count: ::core::option::Option<::prost::alloc::boxed::Box<Node>>,
    #[prost(enumeration="LimitOption", tag="30")]
    pub limit_option: i32,
    #[prost(message, repeated, tag="31")]
    pub row_marks: ::prost::alloc::vec::Vec<Node>,
    #[prost(message, optional, boxed, tag="32")]
    pub set_operations: ::core::option::Option<::prost::alloc::boxed::Box<Node>>,
    #[prost(message, repeated, tag="33")]
    pub constraint_deps: ::prost::alloc::vec::Vec<Node>,
    #[prost(message, repeated, tag="34")]
    pub with_check_options: ::prost::alloc::vec::Vec<Node>,
    #[prost(int32, tag="35")]
    pub stmt_location: i32,
    #[prost(int32, tag="36")]
    pub stmt_len: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InsertStmt {
    #[prost(message, optional, tag="1")]
    pub relation: ::core::option::Option<RangeVar>,
    #[prost(message, repeated, tag="2")]
    pub cols: ::prost::alloc::vec::Vec<Node>,
    #[prost(message, optional, boxed, tag="3")]
    pub select_stmt: ::core::option::Option<::prost::alloc::boxed::Box<Node>>,
    #[prost(message, optional, boxed, tag="4")]
    pub on_conflict_clause: ::core::option::Option<::prost::alloc::boxed::Box<OnConflictClause>>,
    #[prost(message, repeated, tag="5")]
    pub returning_list: ::prost::alloc::vec::Vec<Node>,
    #[prost(message, optional, tag="6")]
    pub with_clause: ::core::option::Option<WithClause>,
    #[prost(enumeration="OverridingKind", tag="7")]
    pub r#override: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteStmt {
    #[prost(message, optional, tag="1")]
    pub relation: ::core::option::Option<RangeVar>,
    #[prost(message, repeated, tag="2")]
    pub using_clause: ::prost::alloc::vec::Vec<Node>,
    #[prost(message, optional, boxed, tag="3")]
    pub where_clause: ::core::option::Option<::prost::alloc::boxed::Box<Node>>,
    #[prost(message, repeated, tag="4")]
    pub returning_list: ::prost::alloc::vec::Vec<Node>,
    #[prost(message, optional, tag="5")]
    pub with_clause: ::core::option::Option<WithClause>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateStmt {
    #[prost(message, optional, tag="1")]
    pub relation: ::core::option::Option<RangeVar>,
    #[prost(message, repeated, tag="2")]
    pub target_list: ::prost::alloc::vec::Vec<Node>,
    #[prost(message, optional, boxed, tag="3")]
    pub where_clause: ::core::option::Option<::prost::alloc::boxed::Box<Node>>,
    #[prost(message, repeated, tag="4")]
    pub from_clause: ::prost::alloc::vec::Vec<Node>,
    #[prost(message, repeated, tag="5")]
    pub returning_list: ::prost::alloc::vec::Vec<Node>,
    #[prost(message, optional, tag="6")]
    pub with_clause: ::core::option::Option<WithClause>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SelectStmt {
    #[prost(message, repeated, tag="1")]
    pub distinct_clause: ::prost::alloc::vec::Vec<Node>,
    #[prost(message, optional, boxed, tag="2")]
    pub into_clause: ::core::option::Option<::prost::alloc::boxed::Box<IntoClause>>,
    #[prost(message, repeated, tag="3")]
    pub target_list: ::prost::alloc::vec::Vec<Node>,
    #[prost(message, repeated, tag="4")]
    pub from_clause: ::prost::alloc::vec::Vec<Node>,
    #[prost(message, optional, boxed, tag="5")]
    pub where_clause: ::core::option::Option<::prost::alloc::boxed::Box<Node>>,
    #[prost(message, repeated, tag="6")]
    pub group_clause: ::prost::alloc::vec::Vec<Node>,
    #[prost(message, optional, boxed, tag="7")]
    pub having_clause: ::core::option::Option<::prost::alloc::boxed::Box<Node>>,
    #[prost(message, repeated, tag="8")]
    pub window_clause: ::prost::alloc::vec::Vec<Node>,
    #[prost(message, repeated, tag="9")]
    pub values_lists: ::prost::alloc::vec::Vec<Node>,
    #[prost(message, repeated, tag="10")]
    pub sort_clause: ::prost::alloc::vec::Vec<Node>,
    #[prost(message, optional, boxed, tag="11")]
    pub limit_offset: ::core::option::Option<::prost::alloc::boxed::Box<Node>>,
    #[prost(message, optional, boxed, tag="12")]
    pub limit_count: ::core::option::Option<::prost::alloc::boxed::Box<Node>>,
    #[prost(enumeration="LimitOption", tag="13")]
    pub limit_option: i32,
    #[prost(message, repeated, tag="14")]
    pub locking_clause: ::prost::alloc::vec::Vec<Node>,
    #[prost(message, optional, tag="15")]
    pub with_clause: ::core::option::Option<WithClause>,
    #[prost(enumeration="SetOperation", tag="16")]
    pub op: i32,
    #[prost(bool, tag="17")]
    pub all: bool,
    #[prost(message, optional, boxed, tag="18")]
    pub larg: ::core::option::Option<::prost::alloc::boxed::Box<SelectStmt>>,
    #[prost(message, optional, boxed, tag="19")]
    pub rarg: ::core::option::Option<::prost::alloc::boxed::Box<SelectStmt>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AlterTableStmt {
    #[prost(message, optional, tag="1")]
    pub relation: ::core::option::Option<RangeVar>,
    #[prost(message, repeated, tag="2")]
    pub cmds: ::prost::alloc::vec::Vec<Node>,
    #[prost(enumeration="ObjectType", tag="3")]
    pub relkind: i32,
    #[prost(bool, tag="4")]
    pub missing_ok: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AlterTableCmd {
    #[prost(enumeration="AlterTableType", tag="1")]
    pub subtype: i32,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    #[prost(int32, tag="3")]
    pub num: i32,
    #[prost(message, optional, tag="4")]
    pub newowner: ::core::option::Option<RoleSpec>,
    #[prost(message, optional, boxed, tag="5")]
    pub def: ::core::option::Option<::prost::alloc::boxed::Box<Node>>,
    #[prost(enumeration="DropBehavior", tag="6")]
    pub behavior: i32,
    #[prost(bool, tag="7")]
    pub missing_ok: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AlterDomainStmt {
    #[prost(string, tag="1")]
    pub subtype: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="2")]
    pub type_name: ::prost::alloc::vec::Vec<Node>,
    #[prost(string, tag="3")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, optional, boxed, tag="4")]
    pub def: ::core::option::Option<::prost::alloc::boxed::Box<Node>>,
    #[prost(enumeration="DropBehavior", tag="5")]
    pub behavior: i32,
    #[prost(bool, tag="6")]
    pub missing_ok: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetOperationStmt {
    #[prost(enumeration="SetOperation", tag="1")]
    pub op: i32,
    #[prost(bool, tag="2")]
    pub all: bool,
    #[prost(message, optional, boxed, tag="3")]
    pub larg: ::core::option::Option<::prost::alloc::boxed::Box<Node>>,
    #[prost(message, optional, boxed, tag="4")]
    pub rarg: ::core::option::Option<::prost::alloc::boxed::Box<Node>>,
    #[prost(message, repeated, tag="5")]
    pub col_types: ::prost::alloc::vec::Vec<Node>,
    #[prost(message, repeated, tag="6")]
    pub col_typmods: ::prost::alloc::vec::Vec<Node>,
    #[prost(message, repeated, tag="7")]
    pub col_collations: ::prost::alloc::vec::Vec<Node>,
    #[prost(message, repeated, tag="8")]
    pub group_clauses: ::prost::alloc::vec::Vec<Node>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GrantStmt {
    #[prost(bool, tag="1")]
    pub is_grant: bool,
    #[prost(enumeration="GrantTargetType", tag="2")]
    pub targtype: i32,
    #[prost(enumeration="ObjectType", tag="3")]
    pub objtype: i32,
    #[prost(message, repeated, tag="4")]
    pub objects: ::prost::alloc::vec::Vec<Node>,
    #[prost(message, repeated, tag="5")]
    pub privileges: ::prost::alloc::vec::Vec<Node>,
    #[prost(message, repeated, tag="6")]
    pub grantees: ::prost::alloc::vec::Vec<Node>,
    #[prost(bool, tag="7")]
    pub grant_option: bool,
    #[prost(enumeration="DropBehavior", tag="8")]
    pub behavior: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GrantRoleStmt {
    #[prost(message, repeated, tag="1")]
    pub granted_roles: ::prost::alloc::vec::Vec<Node>,
    #[prost(message, repeated, tag="2")]
    pub grantee_roles: ::prost::alloc::vec::Vec<Node>,
    #[prost(bool, tag="3")]
    pub is_grant: bool,
    #[prost(bool, tag="4")]
    pub admin_opt: bool,
    #[prost(message, optional, tag="5")]
    pub grantor: ::core::option::Option<RoleSpec>,
    #[prost(enumeration="DropBehavior", tag="6")]
    pub behavior: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AlterDefaultPrivilegesStmt {
    #[prost(message, repeated, tag="1")]
    pub options: ::prost::alloc::vec::Vec<Node>,
    #[prost(message, optional, tag="2")]
    pub action: ::core::option::Option<GrantStmt>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClosePortalStmt {
    #[prost(string, tag="1")]
    pub portalname: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClusterStmt {
    #[prost(message, optional, tag="1")]
    pub relation: ::core::option::Option<RangeVar>,
    #[prost(string, tag="2")]
    pub indexname: ::prost::alloc::string::String,
    #[prost(int32, tag="3")]
    pub options: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CopyStmt {
    #[prost(message, optional, tag="1")]
    pub relation: ::core::option::Option<RangeVar>,
    #[prost(message, optional, boxed, tag="2")]
    pub query: ::core::option::Option<::prost::alloc::boxed::Box<Node>>,
    #[prost(message, repeated, tag="3")]
    pub attlist: ::prost::alloc::vec::Vec<Node>,
    #[prost(bool, tag="4")]
    pub is_from: bool,
    #[prost(bool, tag="5")]
    pub is_program: bool,
    #[prost(string, tag="6")]
    pub filename: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="7")]
    pub options: ::prost::alloc::vec::Vec<Node>,
    #[prost(message, optional, boxed, tag="8")]
    pub where_clause: ::core::option::Option<::prost::alloc::boxed::Box<Node>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateStmt {
    #[prost(message, optional, tag="1")]
    pub relation: ::core::option::Option<RangeVar>,
    #[prost(message, repeated, tag="2")]
    pub table_elts: ::prost::alloc::vec::Vec<Node>,
    #[prost(message, repeated, tag="3")]
    pub inh_relations: ::prost::alloc::vec::Vec<Node>,
    #[prost(message, optional, tag="4")]
    pub partbound: ::core::option::Option<PartitionBoundSpec>,
    #[prost(message, optional, tag="5")]
    pub partspec: ::core::option::Option<PartitionSpec>,
    #[prost(message, optional, tag="6")]
    pub of_typename: ::core::option::Option<TypeName>,
    #[prost(message, repeated, tag="7")]
    pub constraints: ::prost::alloc::vec::Vec<Node>,
    #[prost(message, repeated, tag="8")]
    pub options: ::prost::alloc::vec::Vec<Node>,
    #[prost(enumeration="OnCommitAction", tag="9")]
    pub oncommit: i32,
    #[prost(string, tag="10")]
    pub tablespacename: ::prost::alloc::string::String,
    #[prost(string, tag="11")]
    pub access_method: ::prost::alloc::string::String,
    #[prost(bool, tag="12")]
    pub if_not_exists: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DefineStmt {
    #[prost(enumeration="ObjectType", tag="1")]
    pub kind: i32,
    #[prost(bool, tag="2")]
    pub oldstyle: bool,
    #[prost(message, repeated, tag="3")]
    pub defnames: ::prost::alloc::vec::Vec<Node>,
    #[prost(message, repeated, tag="4")]
    pub args: ::prost::alloc::vec::Vec<Node>,
    #[prost(message, repeated, tag="5")]
    pub definition: ::prost::alloc::vec::Vec<Node>,
    #[prost(bool, tag="6")]
    pub if_not_exists: bool,
    #[prost(bool, tag="7")]
    pub replace: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DropStmt {
    #[prost(message, repeated, tag="1")]
    pub objects: ::prost::alloc::vec::Vec<Node>,
    #[prost(enumeration="ObjectType", tag="2")]
    pub remove_type: i32,
    #[prost(enumeration="DropBehavior", tag="3")]
    pub behavior: i32,
    #[prost(bool, tag="4")]
    pub missing_ok: bool,
    #[prost(bool, tag="5")]
    pub concurrent: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TruncateStmt {
    #[prost(message, repeated, tag="1")]
    pub relations: ::prost::alloc::vec::Vec<Node>,
    #[prost(bool, tag="2")]
    pub restart_seqs: bool,
    #[prost(enumeration="DropBehavior", tag="3")]
    pub behavior: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommentStmt {
    #[prost(enumeration="ObjectType", tag="1")]
    pub objtype: i32,
    #[prost(message, optional, boxed, tag="2")]
    pub object: ::core::option::Option<::prost::alloc::boxed::Box<Node>>,
    #[prost(string, tag="3")]
    pub comment: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FetchStmt {
    #[prost(enumeration="FetchDirection", tag="1")]
    pub direction: i32,
    #[prost(int64, tag="2")]
    pub how_many: i64,
    #[prost(string, tag="3")]
    pub portalname: ::prost::alloc::string::String,
    #[prost(bool, tag="4")]
    pub ismove: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IndexStmt {
    #[prost(string, tag="1")]
    pub idxname: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub relation: ::core::option::Option<RangeVar>,
    #[prost(string, tag="3")]
    pub access_method: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub table_space: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="5")]
    pub index_params: ::prost::alloc::vec::Vec<Node>,
    #[prost(message, repeated, tag="6")]
    pub index_including_params: ::prost::alloc::vec::Vec<Node>,
    #[prost(message, repeated, tag="7")]
    pub options: ::prost::alloc::vec::Vec<Node>,
    #[prost(message, optional, boxed, tag="8")]
    pub where_clause: ::core::option::Option<::prost::alloc::boxed::Box<Node>>,
    #[prost(message, repeated, tag="9")]
    pub exclude_op_names: ::prost::alloc::vec::Vec<Node>,
    #[prost(string, tag="10")]
    pub idxcomment: ::prost::alloc::string::String,
    #[prost(uint32, tag="11")]
    pub index_oid: u32,
    #[prost(uint32, tag="12")]
    pub old_node: u32,
    #[prost(uint32, tag="13")]
    pub old_create_subid: u32,
    #[prost(uint32, tag="14")]
    pub old_first_relfilenode_subid: u32,
    #[prost(bool, tag="15")]
    pub unique: bool,
    #[prost(bool, tag="16")]
    pub primary: bool,
    #[prost(bool, tag="17")]
    pub isconstraint: bool,
    #[prost(bool, tag="18")]
    pub deferrable: bool,
    #[prost(bool, tag="19")]
    pub initdeferred: bool,
    #[prost(bool, tag="20")]
    pub transformed: bool,
    #[prost(bool, tag="21")]
    pub concurrent: bool,
    #[prost(bool, tag="22")]
    pub if_not_exists: bool,
    #[prost(bool, tag="23")]
    pub reset_default_tblspc: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateFunctionStmt {
    #[prost(bool, tag="1")]
    pub is_procedure: bool,
    #[prost(bool, tag="2")]
    pub replace: bool,
    #[prost(message, repeated, tag="3")]
    pub funcname: ::prost::alloc::vec::Vec<Node>,
    #[prost(message, repeated, tag="4")]
    pub parameters: ::prost::alloc::vec::Vec<Node>,
    #[prost(message, optional, tag="5")]
    pub return_type: ::core::option::Option<TypeName>,
    #[prost(message, repeated, tag="6")]
    pub options: ::prost::alloc::vec::Vec<Node>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AlterFunctionStmt {
    #[prost(enumeration="ObjectType", tag="1")]
    pub objtype: i32,
    #[prost(message, optional, tag="2")]
    pub func: ::core::option::Option<ObjectWithArgs>,
    #[prost(message, repeated, tag="3")]
    pub actions: ::prost::alloc::vec::Vec<Node>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DoStmt {
    #[prost(message, repeated, tag="1")]
    pub args: ::prost::alloc::vec::Vec<Node>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RenameStmt {
    #[prost(enumeration="ObjectType", tag="1")]
    pub rename_type: i32,
    #[prost(enumeration="ObjectType", tag="2")]
    pub relation_type: i32,
    #[prost(message, optional, tag="3")]
    pub relation: ::core::option::Option<RangeVar>,
    #[prost(message, optional, boxed, tag="4")]
    pub object: ::core::option::Option<::prost::alloc::boxed::Box<Node>>,
    #[prost(string, tag="5")]
    pub subname: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub newname: ::prost::alloc::string::String,
    #[prost(enumeration="DropBehavior", tag="7")]
    pub behavior: i32,
    #[prost(bool, tag="8")]
    pub missing_ok: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RuleStmt {
    #[prost(message, optional, tag="1")]
    pub relation: ::core::option::Option<RangeVar>,
    #[prost(string, tag="2")]
    pub rulename: ::prost::alloc::string::String,
    #[prost(message, optional, boxed, tag="3")]
    pub where_clause: ::core::option::Option<::prost::alloc::boxed::Box<Node>>,
    #[prost(enumeration="CmdType", tag="4")]
    pub event: i32,
    #[prost(bool, tag="5")]
    pub instead: bool,
    #[prost(message, repeated, tag="6")]
    pub actions: ::prost::alloc::vec::Vec<Node>,
    #[prost(bool, tag="7")]
    pub replace: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NotifyStmt {
    #[prost(string, tag="1")]
    pub conditionname: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub payload: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListenStmt {
    #[prost(string, tag="1")]
    pub conditionname: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnlistenStmt {
    #[prost(string, tag="1")]
    pub conditionname: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionStmt {
    #[prost(enumeration="TransactionStmtKind", tag="1")]
    pub kind: i32,
    #[prost(message, repeated, tag="2")]
    pub options: ::prost::alloc::vec::Vec<Node>,
    #[prost(string, tag="3")]
    pub savepoint_name: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub gid: ::prost::alloc::string::String,
    #[prost(bool, tag="5")]
    pub chain: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ViewStmt {
    #[prost(message, optional, tag="1")]
    pub view: ::core::option::Option<RangeVar>,
    #[prost(message, repeated, tag="2")]
    pub aliases: ::prost::alloc::vec::Vec<Node>,
    #[prost(message, optional, boxed, tag="3")]
    pub query: ::core::option::Option<::prost::alloc::boxed::Box<Node>>,
    #[prost(bool, tag="4")]
    pub replace: bool,
    #[prost(message, repeated, tag="5")]
    pub options: ::prost::alloc::vec::Vec<Node>,
    #[prost(enumeration="ViewCheckOption", tag="6")]
    pub with_check_option: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LoadStmt {
    #[prost(string, tag="1")]
    pub filename: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateDomainStmt {
    #[prost(message, repeated, tag="1")]
    pub domainname: ::prost::alloc::vec::Vec<Node>,
    #[prost(message, optional, tag="2")]
    pub type_name: ::core::option::Option<TypeName>,
    #[prost(message, optional, boxed, tag="3")]
    pub coll_clause: ::core::option::Option<::prost::alloc::boxed::Box<CollateClause>>,
    #[prost(message, repeated, tag="4")]
    pub constraints: ::prost::alloc::vec::Vec<Node>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreatedbStmt {
    #[prost(string, tag="1")]
    pub dbname: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="2")]
    pub options: ::prost::alloc::vec::Vec<Node>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DropdbStmt {
    #[prost(string, tag="1")]
    pub dbname: ::prost::alloc::string::String,
    #[prost(bool, tag="2")]
    pub missing_ok: bool,
    #[prost(message, repeated, tag="3")]
    pub options: ::prost::alloc::vec::Vec<Node>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VacuumStmt {
    #[prost(message, repeated, tag="1")]
    pub options: ::prost::alloc::vec::Vec<Node>,
    #[prost(message, repeated, tag="2")]
    pub rels: ::prost::alloc::vec::Vec<Node>,
    #[prost(bool, tag="3")]
    pub is_vacuumcmd: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExplainStmt {
    #[prost(message, optional, boxed, tag="1")]
    pub query: ::core::option::Option<::prost::alloc::boxed::Box<Node>>,
    #[prost(message, repeated, tag="2")]
    pub options: ::prost::alloc::vec::Vec<Node>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateTableAsStmt {
    #[prost(message, optional, boxed, tag="1")]
    pub query: ::core::option::Option<::prost::alloc::boxed::Box<Node>>,
    #[prost(message, optional, boxed, tag="2")]
    pub into: ::core::option::Option<::prost::alloc::boxed::Box<IntoClause>>,
    #[prost(enumeration="ObjectType", tag="3")]
    pub relkind: i32,
    #[prost(bool, tag="4")]
    pub is_select_into: bool,
    #[prost(bool, tag="5")]
    pub if_not_exists: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateSeqStmt {
    #[prost(message, optional, tag="1")]
    pub sequence: ::core::option::Option<RangeVar>,
    #[prost(message, repeated, tag="2")]
    pub options: ::prost::alloc::vec::Vec<Node>,
    #[prost(uint32, tag="3")]
    pub owner_id: u32,
    #[prost(bool, tag="4")]
    pub for_identity: bool,
    #[prost(bool, tag="5")]
    pub if_not_exists: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AlterSeqStmt {
    #[prost(message, optional, tag="1")]
    pub sequence: ::core::option::Option<RangeVar>,
    #[prost(message, repeated, tag="2")]
    pub options: ::prost::alloc::vec::Vec<Node>,
    #[prost(bool, tag="3")]
    pub for_identity: bool,
    #[prost(bool, tag="4")]
    pub missing_ok: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VariableSetStmt {
    #[prost(enumeration="VariableSetKind", tag="1")]
    pub kind: i32,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="3")]
    pub args: ::prost::alloc::vec::Vec<Node>,
    #[prost(bool, tag="4")]
    pub is_local: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VariableShowStmt {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DiscardStmt {
    #[prost(enumeration="DiscardMode", tag="1")]
    pub target: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateTrigStmt {
    #[prost(string, tag="1")]
    pub trigname: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub relation: ::core::option::Option<RangeVar>,
    #[prost(message, repeated, tag="3")]
    pub funcname: ::prost::alloc::vec::Vec<Node>,
    #[prost(message, repeated, tag="4")]
    pub args: ::prost::alloc::vec::Vec<Node>,
    #[prost(bool, tag="5")]
    pub row: bool,
    #[prost(int32, tag="6")]
    pub timing: i32,
    #[prost(int32, tag="7")]
    pub events: i32,
    #[prost(message, repeated, tag="8")]
    pub columns: ::prost::alloc::vec::Vec<Node>,
    #[prost(message, optional, boxed, tag="9")]
    pub when_clause: ::core::option::Option<::prost::alloc::boxed::Box<Node>>,
    #[prost(bool, tag="10")]
    pub isconstraint: bool,
    #[prost(message, repeated, tag="11")]
    pub transition_rels: ::prost::alloc::vec::Vec<Node>,
    #[prost(bool, tag="12")]
    pub deferrable: bool,
    #[prost(bool, tag="13")]
    pub initdeferred: bool,
    #[prost(message, optional, tag="14")]
    pub constrrel: ::core::option::Option<RangeVar>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreatePLangStmt {
    #[prost(bool, tag="1")]
    pub replace: bool,
    #[prost(string, tag="2")]
    pub plname: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="3")]
    pub plhandler: ::prost::alloc::vec::Vec<Node>,
    #[prost(message, repeated, tag="4")]
    pub plinline: ::prost::alloc::vec::Vec<Node>,
    #[prost(message, repeated, tag="5")]
    pub plvalidator: ::prost::alloc::vec::Vec<Node>,
    #[prost(bool, tag="6")]
    pub pltrusted: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateRoleStmt {
    #[prost(enumeration="RoleStmtType", tag="1")]
    pub stmt_type: i32,
    #[prost(string, tag="2")]
    pub role: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="3")]
    pub options: ::prost::alloc::vec::Vec<Node>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AlterRoleStmt {
    #[prost(message, optional, tag="1")]
    pub role: ::core::option::Option<RoleSpec>,
    #[prost(message, repeated, tag="2")]
    pub options: ::prost::alloc::vec::Vec<Node>,
    #[prost(int32, tag="3")]
    pub action: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DropRoleStmt {
    #[prost(message, repeated, tag="1")]
    pub roles: ::prost::alloc::vec::Vec<Node>,
    #[prost(bool, tag="2")]
    pub missing_ok: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LockStmt {
    #[prost(message, repeated, tag="1")]
    pub relations: ::prost::alloc::vec::Vec<Node>,
    #[prost(int32, tag="2")]
    pub mode: i32,
    #[prost(bool, tag="3")]
    pub nowait: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConstraintsSetStmt {
    #[prost(message, repeated, tag="1")]
    pub constraints: ::prost::alloc::vec::Vec<Node>,
    #[prost(bool, tag="2")]
    pub deferred: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReindexStmt {
    #[prost(enumeration="ReindexObjectType", tag="1")]
    pub kind: i32,
    #[prost(message, optional, tag="2")]
    pub relation: ::core::option::Option<RangeVar>,
    #[prost(string, tag="3")]
    pub name: ::prost::alloc::string::String,
    #[prost(int32, tag="4")]
    pub options: i32,
    #[prost(bool, tag="5")]
    pub concurrent: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckPointStmt {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateSchemaStmt {
    #[prost(string, tag="1")]
    pub schemaname: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub authrole: ::core::option::Option<RoleSpec>,
    #[prost(message, repeated, tag="3")]
    pub schema_elts: ::prost::alloc::vec::Vec<Node>,
    #[prost(bool, tag="4")]
    pub if_not_exists: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AlterDatabaseStmt {
    #[prost(string, tag="1")]
    pub dbname: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="2")]
    pub options: ::prost::alloc::vec::Vec<Node>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AlterDatabaseSetStmt {
    #[prost(string, tag="1")]
    pub dbname: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub setstmt: ::core::option::Option<VariableSetStmt>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AlterRoleSetStmt {
    #[prost(message, optional, tag="1")]
    pub role: ::core::option::Option<RoleSpec>,
    #[prost(string, tag="2")]
    pub database: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub setstmt: ::core::option::Option<VariableSetStmt>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateConversionStmt {
    #[prost(message, repeated, tag="1")]
    pub conversion_name: ::prost::alloc::vec::Vec<Node>,
    #[prost(string, tag="2")]
    pub for_encoding_name: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub to_encoding_name: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="4")]
    pub func_name: ::prost::alloc::vec::Vec<Node>,
    #[prost(bool, tag="5")]
    pub def: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateCastStmt {
    #[prost(message, optional, tag="1")]
    pub sourcetype: ::core::option::Option<TypeName>,
    #[prost(message, optional, tag="2")]
    pub targettype: ::core::option::Option<TypeName>,
    #[prost(message, optional, tag="3")]
    pub func: ::core::option::Option<ObjectWithArgs>,
    #[prost(enumeration="CoercionContext", tag="4")]
    pub context: i32,
    #[prost(bool, tag="5")]
    pub inout: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateOpClassStmt {
    #[prost(message, repeated, tag="1")]
    pub opclassname: ::prost::alloc::vec::Vec<Node>,
    #[prost(message, repeated, tag="2")]
    pub opfamilyname: ::prost::alloc::vec::Vec<Node>,
    #[prost(string, tag="3")]
    pub amname: ::prost::alloc::string::String,
    #[prost(message, optional, tag="4")]
    pub datatype: ::core::option::Option<TypeName>,
    #[prost(message, repeated, tag="5")]
    pub items: ::prost::alloc::vec::Vec<Node>,
    #[prost(bool, tag="6")]
    pub is_default: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateOpFamilyStmt {
    #[prost(message, repeated, tag="1")]
    pub opfamilyname: ::prost::alloc::vec::Vec<Node>,
    #[prost(string, tag="2")]
    pub amname: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AlterOpFamilyStmt {
    #[prost(message, repeated, tag="1")]
    pub opfamilyname: ::prost::alloc::vec::Vec<Node>,
    #[prost(string, tag="2")]
    pub amname: ::prost::alloc::string::String,
    #[prost(bool, tag="3")]
    pub is_drop: bool,
    #[prost(message, repeated, tag="4")]
    pub items: ::prost::alloc::vec::Vec<Node>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrepareStmt {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="2")]
    pub argtypes: ::prost::alloc::vec::Vec<Node>,
    #[prost(message, optional, boxed, tag="3")]
    pub query: ::core::option::Option<::prost::alloc::boxed::Box<Node>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecuteStmt {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="2")]
    pub params: ::prost::alloc::vec::Vec<Node>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeallocateStmt {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeclareCursorStmt {
    #[prost(string, tag="1")]
    pub portalname: ::prost::alloc::string::String,
    #[prost(int32, tag="2")]
    pub options: i32,
    #[prost(message, optional, boxed, tag="3")]
    pub query: ::core::option::Option<::prost::alloc::boxed::Box<Node>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateTableSpaceStmt {
    #[prost(string, tag="1")]
    pub tablespacename: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub owner: ::core::option::Option<RoleSpec>,
    #[prost(string, tag="3")]
    pub location: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="4")]
    pub options: ::prost::alloc::vec::Vec<Node>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DropTableSpaceStmt {
    #[prost(string, tag="1")]
    pub tablespacename: ::prost::alloc::string::String,
    #[prost(bool, tag="2")]
    pub missing_ok: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AlterObjectDependsStmt {
    #[prost(enumeration="ObjectType", tag="1")]
    pub object_type: i32,
    #[prost(message, optional, tag="2")]
    pub relation: ::core::option::Option<RangeVar>,
    #[prost(message, optional, boxed, tag="3")]
    pub object: ::core::option::Option<::prost::alloc::boxed::Box<Node>>,
    #[prost(message, optional, boxed, tag="4")]
    pub extname: ::core::option::Option<::prost::alloc::boxed::Box<Node>>,
    #[prost(bool, tag="5")]
    pub remove: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AlterObjectSchemaStmt {
    #[prost(enumeration="ObjectType", tag="1")]
    pub object_type: i32,
    #[prost(message, optional, tag="2")]
    pub relation: ::core::option::Option<RangeVar>,
    #[prost(message, optional, boxed, tag="3")]
    pub object: ::core::option::Option<::prost::alloc::boxed::Box<Node>>,
    #[prost(string, tag="4")]
    pub newschema: ::prost::alloc::string::String,
    #[prost(bool, tag="5")]
    pub missing_ok: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AlterOwnerStmt {
    #[prost(enumeration="ObjectType", tag="1")]
    pub object_type: i32,
    #[prost(message, optional, tag="2")]
    pub relation: ::core::option::Option<RangeVar>,
    #[prost(message, optional, boxed, tag="3")]
    pub object: ::core::option::Option<::prost::alloc::boxed::Box<Node>>,
    #[prost(message, optional, tag="4")]
    pub newowner: ::core::option::Option<RoleSpec>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AlterOperatorStmt {
    #[prost(message, optional, tag="1")]
    pub opername: ::core::option::Option<ObjectWithArgs>,
    #[prost(message, repeated, tag="2")]
    pub options: ::prost::alloc::vec::Vec<Node>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AlterTypeStmt {
    #[prost(message, repeated, tag="1")]
    pub type_name: ::prost::alloc::vec::Vec<Node>,
    #[prost(message, repeated, tag="2")]
    pub options: ::prost::alloc::vec::Vec<Node>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DropOwnedStmt {
    #[prost(message, repeated, tag="1")]
    pub roles: ::prost::alloc::vec::Vec<Node>,
    #[prost(enumeration="DropBehavior", tag="2")]
    pub behavior: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReassignOwnedStmt {
    #[prost(message, repeated, tag="1")]
    pub roles: ::prost::alloc::vec::Vec<Node>,
    #[prost(message, optional, tag="2")]
    pub newrole: ::core::option::Option<RoleSpec>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CompositeTypeStmt {
    #[prost(message, optional, tag="1")]
    pub typevar: ::core::option::Option<RangeVar>,
    #[prost(message, repeated, tag="2")]
    pub coldeflist: ::prost::alloc::vec::Vec<Node>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateEnumStmt {
    #[prost(message, repeated, tag="1")]
    pub type_name: ::prost::alloc::vec::Vec<Node>,
    #[prost(message, repeated, tag="2")]
    pub vals: ::prost::alloc::vec::Vec<Node>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateRangeStmt {
    #[prost(message, repeated, tag="1")]
    pub type_name: ::prost::alloc::vec::Vec<Node>,
    #[prost(message, repeated, tag="2")]
    pub params: ::prost::alloc::vec::Vec<Node>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AlterEnumStmt {
    #[prost(message, repeated, tag="1")]
    pub type_name: ::prost::alloc::vec::Vec<Node>,
    #[prost(string, tag="2")]
    pub old_val: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub new_val: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub new_val_neighbor: ::prost::alloc::string::String,
    #[prost(bool, tag="5")]
    pub new_val_is_after: bool,
    #[prost(bool, tag="6")]
    pub skip_if_new_val_exists: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AlterTsDictionaryStmt {
    #[prost(message, repeated, tag="1")]
    pub dictname: ::prost::alloc::vec::Vec<Node>,
    #[prost(message, repeated, tag="2")]
    pub options: ::prost::alloc::vec::Vec<Node>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AlterTsConfigurationStmt {
    #[prost(enumeration="AlterTsConfigType", tag="1")]
    pub kind: i32,
    #[prost(message, repeated, tag="2")]
    pub cfgname: ::prost::alloc::vec::Vec<Node>,
    #[prost(message, repeated, tag="3")]
    pub tokentype: ::prost::alloc::vec::Vec<Node>,
    #[prost(message, repeated, tag="4")]
    pub dicts: ::prost::alloc::vec::Vec<Node>,
    #[prost(bool, tag="5")]
    pub r#override: bool,
    #[prost(bool, tag="6")]
    pub replace: bool,
    #[prost(bool, tag="7")]
    pub missing_ok: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateFdwStmt {
    #[prost(string, tag="1")]
    pub fdwname: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="2")]
    pub func_options: ::prost::alloc::vec::Vec<Node>,
    #[prost(message, repeated, tag="3")]
    pub options: ::prost::alloc::vec::Vec<Node>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AlterFdwStmt {
    #[prost(string, tag="1")]
    pub fdwname: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="2")]
    pub func_options: ::prost::alloc::vec::Vec<Node>,
    #[prost(message, repeated, tag="3")]
    pub options: ::prost::alloc::vec::Vec<Node>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateForeignServerStmt {
    #[prost(string, tag="1")]
    pub servername: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub servertype: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub version: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub fdwname: ::prost::alloc::string::String,
    #[prost(bool, tag="5")]
    pub if_not_exists: bool,
    #[prost(message, repeated, tag="6")]
    pub options: ::prost::alloc::vec::Vec<Node>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AlterForeignServerStmt {
    #[prost(string, tag="1")]
    pub servername: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub version: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="3")]
    pub options: ::prost::alloc::vec::Vec<Node>,
    #[prost(bool, tag="4")]
    pub has_version: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateUserMappingStmt {
    #[prost(message, optional, tag="1")]
    pub user: ::core::option::Option<RoleSpec>,
    #[prost(string, tag="2")]
    pub servername: ::prost::alloc::string::String,
    #[prost(bool, tag="3")]
    pub if_not_exists: bool,
    #[prost(message, repeated, tag="4")]
    pub options: ::prost::alloc::vec::Vec<Node>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AlterUserMappingStmt {
    #[prost(message, optional, tag="1")]
    pub user: ::core::option::Option<RoleSpec>,
    #[prost(string, tag="2")]
    pub servername: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="3")]
    pub options: ::prost::alloc::vec::Vec<Node>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DropUserMappingStmt {
    #[prost(message, optional, tag="1")]
    pub user: ::core::option::Option<RoleSpec>,
    #[prost(string, tag="2")]
    pub servername: ::prost::alloc::string::String,
    #[prost(bool, tag="3")]
    pub missing_ok: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AlterTableSpaceOptionsStmt {
    #[prost(string, tag="1")]
    pub tablespacename: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="2")]
    pub options: ::prost::alloc::vec::Vec<Node>,
    #[prost(bool, tag="3")]
    pub is_reset: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AlterTableMoveAllStmt {
    #[prost(string, tag="1")]
    pub orig_tablespacename: ::prost::alloc::string::String,
    #[prost(enumeration="ObjectType", tag="2")]
    pub objtype: i32,
    #[prost(message, repeated, tag="3")]
    pub roles: ::prost::alloc::vec::Vec<Node>,
    #[prost(string, tag="4")]
    pub new_tablespacename: ::prost::alloc::string::String,
    #[prost(bool, tag="5")]
    pub nowait: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SecLabelStmt {
    #[prost(enumeration="ObjectType", tag="1")]
    pub objtype: i32,
    #[prost(message, optional, boxed, tag="2")]
    pub object: ::core::option::Option<::prost::alloc::boxed::Box<Node>>,
    #[prost(string, tag="3")]
    pub provider: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub label: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateForeignTableStmt {
    #[prost(message, optional, tag="1")]
    pub base_stmt: ::core::option::Option<CreateStmt>,
    #[prost(string, tag="2")]
    pub servername: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="3")]
    pub options: ::prost::alloc::vec::Vec<Node>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportForeignSchemaStmt {
    #[prost(string, tag="1")]
    pub server_name: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub remote_schema: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub local_schema: ::prost::alloc::string::String,
    #[prost(enumeration="ImportForeignSchemaType", tag="4")]
    pub list_type: i32,
    #[prost(message, repeated, tag="5")]
    pub table_list: ::prost::alloc::vec::Vec<Node>,
    #[prost(message, repeated, tag="6")]
    pub options: ::prost::alloc::vec::Vec<Node>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateExtensionStmt {
    #[prost(string, tag="1")]
    pub extname: ::prost::alloc::string::String,
    #[prost(bool, tag="2")]
    pub if_not_exists: bool,
    #[prost(message, repeated, tag="3")]
    pub options: ::prost::alloc::vec::Vec<Node>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AlterExtensionStmt {
    #[prost(string, tag="1")]
    pub extname: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="2")]
    pub options: ::prost::alloc::vec::Vec<Node>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AlterExtensionContentsStmt {
    #[prost(string, tag="1")]
    pub extname: ::prost::alloc::string::String,
    #[prost(int32, tag="2")]
    pub action: i32,
    #[prost(enumeration="ObjectType", tag="3")]
    pub objtype: i32,
    #[prost(message, optional, boxed, tag="4")]
    pub object: ::core::option::Option<::prost::alloc::boxed::Box<Node>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateEventTrigStmt {
    #[prost(string, tag="1")]
    pub trigname: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub eventname: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="3")]
    pub whenclause: ::prost::alloc::vec::Vec<Node>,
    #[prost(message, repeated, tag="4")]
    pub funcname: ::prost::alloc::vec::Vec<Node>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AlterEventTrigStmt {
    #[prost(string, tag="1")]
    pub trigname: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub tgenabled: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RefreshMatViewStmt {
    #[prost(bool, tag="1")]
    pub concurrent: bool,
    #[prost(bool, tag="2")]
    pub skip_data: bool,
    #[prost(message, optional, tag="3")]
    pub relation: ::core::option::Option<RangeVar>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReplicaIdentityStmt {
    #[prost(string, tag="1")]
    pub identity_type: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AlterSystemStmt {
    #[prost(message, optional, tag="1")]
    pub setstmt: ::core::option::Option<VariableSetStmt>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreatePolicyStmt {
    #[prost(string, tag="1")]
    pub policy_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub table: ::core::option::Option<RangeVar>,
    #[prost(string, tag="3")]
    pub cmd_name: ::prost::alloc::string::String,
    #[prost(bool, tag="4")]
    pub permissive: bool,
    #[prost(message, repeated, tag="5")]
    pub roles: ::prost::alloc::vec::Vec<Node>,
    #[prost(message, optional, boxed, tag="6")]
    pub qual: ::core::option::Option<::prost::alloc::boxed::Box<Node>>,
    #[prost(message, optional, boxed, tag="7")]
    pub with_check: ::core::option::Option<::prost::alloc::boxed::Box<Node>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AlterPolicyStmt {
    #[prost(string, tag="1")]
    pub policy_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub table: ::core::option::Option<RangeVar>,
    #[prost(message, repeated, tag="3")]
    pub roles: ::prost::alloc::vec::Vec<Node>,
    #[prost(message, optional, boxed, tag="4")]
    pub qual: ::core::option::Option<::prost::alloc::boxed::Box<Node>>,
    #[prost(message, optional, boxed, tag="5")]
    pub with_check: ::core::option::Option<::prost::alloc::boxed::Box<Node>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateTransformStmt {
    #[prost(bool, tag="1")]
    pub replace: bool,
    #[prost(message, optional, tag="2")]
    pub type_name: ::core::option::Option<TypeName>,
    #[prost(string, tag="3")]
    pub lang: ::prost::alloc::string::String,
    #[prost(message, optional, tag="4")]
    pub fromsql: ::core::option::Option<ObjectWithArgs>,
    #[prost(message, optional, tag="5")]
    pub tosql: ::core::option::Option<ObjectWithArgs>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateAmStmt {
    #[prost(string, tag="1")]
    pub amname: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="2")]
    pub handler_name: ::prost::alloc::vec::Vec<Node>,
    #[prost(string, tag="3")]
    pub amtype: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreatePublicationStmt {
    #[prost(string, tag="1")]
    pub pubname: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="2")]
    pub options: ::prost::alloc::vec::Vec<Node>,
    #[prost(message, repeated, tag="3")]
    pub tables: ::prost::alloc::vec::Vec<Node>,
    #[prost(bool, tag="4")]
    pub for_all_tables: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AlterPublicationStmt {
    #[prost(string, tag="1")]
    pub pubname: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="2")]
    pub options: ::prost::alloc::vec::Vec<Node>,
    #[prost(message, repeated, tag="3")]
    pub tables: ::prost::alloc::vec::Vec<Node>,
    #[prost(bool, tag="4")]
    pub for_all_tables: bool,
    #[prost(enumeration="DefElemAction", tag="5")]
    pub table_action: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateSubscriptionStmt {
    #[prost(string, tag="1")]
    pub subname: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub conninfo: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="3")]
    pub publication: ::prost::alloc::vec::Vec<Node>,
    #[prost(message, repeated, tag="4")]
    pub options: ::prost::alloc::vec::Vec<Node>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AlterSubscriptionStmt {
    #[prost(enumeration="AlterSubscriptionType", tag="1")]
    pub kind: i32,
    #[prost(string, tag="2")]
    pub subname: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub conninfo: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="4")]
    pub publication: ::prost::alloc::vec::Vec<Node>,
    #[prost(message, repeated, tag="5")]
    pub options: ::prost::alloc::vec::Vec<Node>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DropSubscriptionStmt {
    #[prost(string, tag="1")]
    pub subname: ::prost::alloc::string::String,
    #[prost(bool, tag="2")]
    pub missing_ok: bool,
    #[prost(enumeration="DropBehavior", tag="3")]
    pub behavior: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateStatsStmt {
    #[prost(message, repeated, tag="1")]
    pub defnames: ::prost::alloc::vec::Vec<Node>,
    #[prost(message, repeated, tag="2")]
    pub stat_types: ::prost::alloc::vec::Vec<Node>,
    #[prost(message, repeated, tag="3")]
    pub exprs: ::prost::alloc::vec::Vec<Node>,
    #[prost(message, repeated, tag="4")]
    pub relations: ::prost::alloc::vec::Vec<Node>,
    #[prost(string, tag="5")]
    pub stxcomment: ::prost::alloc::string::String,
    #[prost(bool, tag="6")]
    pub if_not_exists: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AlterCollationStmt {
    #[prost(message, repeated, tag="1")]
    pub collname: ::prost::alloc::vec::Vec<Node>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CallStmt {
    #[prost(message, optional, boxed, tag="1")]
    pub funccall: ::core::option::Option<::prost::alloc::boxed::Box<FuncCall>>,
    #[prost(message, optional, boxed, tag="2")]
    pub funcexpr: ::core::option::Option<::prost::alloc::boxed::Box<FuncExpr>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AlterStatsStmt {
    #[prost(message, repeated, tag="1")]
    pub defnames: ::prost::alloc::vec::Vec<Node>,
    #[prost(int32, tag="2")]
    pub stxstattarget: i32,
    #[prost(bool, tag="3")]
    pub missing_ok: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AExpr {
    #[prost(enumeration="AExprKind", tag="1")]
    pub kind: i32,
    #[prost(message, repeated, tag="2")]
    pub name: ::prost::alloc::vec::Vec<Node>,
    #[prost(message, optional, boxed, tag="3")]
    pub lexpr: ::core::option::Option<::prost::alloc::boxed::Box<Node>>,
    #[prost(message, optional, boxed, tag="4")]
    pub rexpr: ::core::option::Option<::prost::alloc::boxed::Box<Node>>,
    #[prost(int32, tag="5")]
    pub location: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ColumnRef {
    #[prost(message, repeated, tag="1")]
    pub fields: ::prost::alloc::vec::Vec<Node>,
    #[prost(int32, tag="2")]
    pub location: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ParamRef {
    #[prost(int32, tag="1")]
    pub number: i32,
    #[prost(int32, tag="2")]
    pub location: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AConst {
    #[prost(message, optional, boxed, tag="1")]
    pub val: ::core::option::Option<::prost::alloc::boxed::Box<Node>>,
    #[prost(int32, tag="2")]
    pub location: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FuncCall {
    #[prost(message, repeated, tag="1")]
    pub funcname: ::prost::alloc::vec::Vec<Node>,
    #[prost(message, repeated, tag="2")]
    pub args: ::prost::alloc::vec::Vec<Node>,
    #[prost(message, repeated, tag="3")]
    pub agg_order: ::prost::alloc::vec::Vec<Node>,
    #[prost(message, optional, boxed, tag="4")]
    pub agg_filter: ::core::option::Option<::prost::alloc::boxed::Box<Node>>,
    #[prost(bool, tag="5")]
    pub agg_within_group: bool,
    #[prost(bool, tag="6")]
    pub agg_star: bool,
    #[prost(bool, tag="7")]
    pub agg_distinct: bool,
    #[prost(bool, tag="8")]
    pub func_variadic: bool,
    #[prost(message, optional, boxed, tag="9")]
    pub over: ::core::option::Option<::prost::alloc::boxed::Box<WindowDef>>,
    #[prost(int32, tag="10")]
    pub location: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AStar {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AIndices {
    #[prost(bool, tag="1")]
    pub is_slice: bool,
    #[prost(message, optional, boxed, tag="2")]
    pub lidx: ::core::option::Option<::prost::alloc::boxed::Box<Node>>,
    #[prost(message, optional, boxed, tag="3")]
    pub uidx: ::core::option::Option<::prost::alloc::boxed::Box<Node>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AIndirection {
    #[prost(message, optional, boxed, tag="1")]
    pub arg: ::core::option::Option<::prost::alloc::boxed::Box<Node>>,
    #[prost(message, repeated, tag="2")]
    pub indirection: ::prost::alloc::vec::Vec<Node>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AArrayExpr {
    #[prost(message, repeated, tag="1")]
    pub elements: ::prost::alloc::vec::Vec<Node>,
    #[prost(int32, tag="2")]
    pub location: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResTarget {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="2")]
    pub indirection: ::prost::alloc::vec::Vec<Node>,
    #[prost(message, optional, boxed, tag="3")]
    pub val: ::core::option::Option<::prost::alloc::boxed::Box<Node>>,
    #[prost(int32, tag="4")]
    pub location: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MultiAssignRef {
    #[prost(message, optional, boxed, tag="1")]
    pub source: ::core::option::Option<::prost::alloc::boxed::Box<Node>>,
    #[prost(int32, tag="2")]
    pub colno: i32,
    #[prost(int32, tag="3")]
    pub ncolumns: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TypeCast {
    #[prost(message, optional, boxed, tag="1")]
    pub arg: ::core::option::Option<::prost::alloc::boxed::Box<Node>>,
    #[prost(message, optional, tag="2")]
    pub type_name: ::core::option::Option<TypeName>,
    #[prost(int32, tag="3")]
    pub location: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CollateClause {
    #[prost(message, optional, boxed, tag="1")]
    pub arg: ::core::option::Option<::prost::alloc::boxed::Box<Node>>,
    #[prost(message, repeated, tag="2")]
    pub collname: ::prost::alloc::vec::Vec<Node>,
    #[prost(int32, tag="3")]
    pub location: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SortBy {
    #[prost(message, optional, boxed, tag="1")]
    pub node: ::core::option::Option<::prost::alloc::boxed::Box<Node>>,
    #[prost(enumeration="SortByDir", tag="2")]
    pub sortby_dir: i32,
    #[prost(enumeration="SortByNulls", tag="3")]
    pub sortby_nulls: i32,
    #[prost(message, repeated, tag="4")]
    pub use_op: ::prost::alloc::vec::Vec<Node>,
    #[prost(int32, tag="5")]
    pub location: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WindowDef {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub refname: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="3")]
    pub partition_clause: ::prost::alloc::vec::Vec<Node>,
    #[prost(message, repeated, tag="4")]
    pub order_clause: ::prost::alloc::vec::Vec<Node>,
    #[prost(int32, tag="5")]
    pub frame_options: i32,
    #[prost(message, optional, boxed, tag="6")]
    pub start_offset: ::core::option::Option<::prost::alloc::boxed::Box<Node>>,
    #[prost(message, optional, boxed, tag="7")]
    pub end_offset: ::core::option::Option<::prost::alloc::boxed::Box<Node>>,
    #[prost(int32, tag="8")]
    pub location: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RangeSubselect {
    #[prost(bool, tag="1")]
    pub lateral: bool,
    #[prost(message, optional, boxed, tag="2")]
    pub subquery: ::core::option::Option<::prost::alloc::boxed::Box<Node>>,
    #[prost(message, optional, tag="3")]
    pub alias: ::core::option::Option<Alias>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RangeFunction {
    #[prost(bool, tag="1")]
    pub lateral: bool,
    #[prost(bool, tag="2")]
    pub ordinality: bool,
    #[prost(bool, tag="3")]
    pub is_rowsfrom: bool,
    #[prost(message, repeated, tag="4")]
    pub functions: ::prost::alloc::vec::Vec<Node>,
    #[prost(message, optional, tag="5")]
    pub alias: ::core::option::Option<Alias>,
    #[prost(message, repeated, tag="6")]
    pub coldeflist: ::prost::alloc::vec::Vec<Node>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RangeTableSample {
    #[prost(message, optional, boxed, tag="1")]
    pub relation: ::core::option::Option<::prost::alloc::boxed::Box<Node>>,
    #[prost(message, repeated, tag="2")]
    pub method: ::prost::alloc::vec::Vec<Node>,
    #[prost(message, repeated, tag="3")]
    pub args: ::prost::alloc::vec::Vec<Node>,
    #[prost(message, optional, boxed, tag="4")]
    pub repeatable: ::core::option::Option<::prost::alloc::boxed::Box<Node>>,
    #[prost(int32, tag="5")]
    pub location: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RangeTableFunc {
    #[prost(bool, tag="1")]
    pub lateral: bool,
    #[prost(message, optional, boxed, tag="2")]
    pub docexpr: ::core::option::Option<::prost::alloc::boxed::Box<Node>>,
    #[prost(message, optional, boxed, tag="3")]
    pub rowexpr: ::core::option::Option<::prost::alloc::boxed::Box<Node>>,
    #[prost(message, repeated, tag="4")]
    pub namespaces: ::prost::alloc::vec::Vec<Node>,
    #[prost(message, repeated, tag="5")]
    pub columns: ::prost::alloc::vec::Vec<Node>,
    #[prost(message, optional, tag="6")]
    pub alias: ::core::option::Option<Alias>,
    #[prost(int32, tag="7")]
    pub location: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RangeTableFuncCol {
    #[prost(string, tag="1")]
    pub colname: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub type_name: ::core::option::Option<TypeName>,
    #[prost(bool, tag="3")]
    pub for_ordinality: bool,
    #[prost(bool, tag="4")]
    pub is_not_null: bool,
    #[prost(message, optional, boxed, tag="5")]
    pub colexpr: ::core::option::Option<::prost::alloc::boxed::Box<Node>>,
    #[prost(message, optional, boxed, tag="6")]
    pub coldefexpr: ::core::option::Option<::prost::alloc::boxed::Box<Node>>,
    #[prost(int32, tag="7")]
    pub location: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TypeName {
    #[prost(message, repeated, tag="1")]
    pub names: ::prost::alloc::vec::Vec<Node>,
    #[prost(uint32, tag="2")]
    pub type_oid: u32,
    #[prost(bool, tag="3")]
    pub setof: bool,
    #[prost(bool, tag="4")]
    pub pct_type: bool,
    #[prost(message, repeated, tag="5")]
    pub typmods: ::prost::alloc::vec::Vec<Node>,
    #[prost(int32, tag="6")]
    pub typemod: i32,
    #[prost(message, repeated, tag="7")]
    pub array_bounds: ::prost::alloc::vec::Vec<Node>,
    #[prost(int32, tag="8")]
    pub location: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ColumnDef {
    #[prost(string, tag="1")]
    pub colname: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub type_name: ::core::option::Option<TypeName>,
    #[prost(int32, tag="3")]
    pub inhcount: i32,
    #[prost(bool, tag="4")]
    pub is_local: bool,
    #[prost(bool, tag="5")]
    pub is_not_null: bool,
    #[prost(bool, tag="6")]
    pub is_from_type: bool,
    #[prost(string, tag="7")]
    pub storage: ::prost::alloc::string::String,
    #[prost(message, optional, boxed, tag="8")]
    pub raw_default: ::core::option::Option<::prost::alloc::boxed::Box<Node>>,
    #[prost(message, optional, boxed, tag="9")]
    pub cooked_default: ::core::option::Option<::prost::alloc::boxed::Box<Node>>,
    #[prost(string, tag="10")]
    pub identity: ::prost::alloc::string::String,
    #[prost(message, optional, tag="11")]
    pub identity_sequence: ::core::option::Option<RangeVar>,
    #[prost(string, tag="12")]
    pub generated: ::prost::alloc::string::String,
    #[prost(message, optional, boxed, tag="13")]
    pub coll_clause: ::core::option::Option<::prost::alloc::boxed::Box<CollateClause>>,
    #[prost(uint32, tag="14")]
    pub coll_oid: u32,
    #[prost(message, repeated, tag="15")]
    pub constraints: ::prost::alloc::vec::Vec<Node>,
    #[prost(message, repeated, tag="16")]
    pub fdwoptions: ::prost::alloc::vec::Vec<Node>,
    #[prost(int32, tag="17")]
    pub location: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IndexElem {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, optional, boxed, tag="2")]
    pub expr: ::core::option::Option<::prost::alloc::boxed::Box<Node>>,
    #[prost(string, tag="3")]
    pub indexcolname: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="4")]
    pub collation: ::prost::alloc::vec::Vec<Node>,
    #[prost(message, repeated, tag="5")]
    pub opclass: ::prost::alloc::vec::Vec<Node>,
    #[prost(message, repeated, tag="6")]
    pub opclassopts: ::prost::alloc::vec::Vec<Node>,
    #[prost(enumeration="SortByDir", tag="7")]
    pub ordering: i32,
    #[prost(enumeration="SortByNulls", tag="8")]
    pub nulls_ordering: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Constraint {
    #[prost(enumeration="ConstrType", tag="1")]
    pub contype: i32,
    #[prost(string, tag="2")]
    pub conname: ::prost::alloc::string::String,
    #[prost(bool, tag="3")]
    pub deferrable: bool,
    #[prost(bool, tag="4")]
    pub initdeferred: bool,
    #[prost(int32, tag="5")]
    pub location: i32,
    #[prost(bool, tag="6")]
    pub is_no_inherit: bool,
    #[prost(message, optional, boxed, tag="7")]
    pub raw_expr: ::core::option::Option<::prost::alloc::boxed::Box<Node>>,
    #[prost(string, tag="8")]
    pub cooked_expr: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub generated_when: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="10")]
    pub keys: ::prost::alloc::vec::Vec<Node>,
    #[prost(message, repeated, tag="11")]
    pub including: ::prost::alloc::vec::Vec<Node>,
    #[prost(message, repeated, tag="12")]
    pub exclusions: ::prost::alloc::vec::Vec<Node>,
    #[prost(message, repeated, tag="13")]
    pub options: ::prost::alloc::vec::Vec<Node>,
    #[prost(string, tag="14")]
    pub indexname: ::prost::alloc::string::String,
    #[prost(string, tag="15")]
    pub indexspace: ::prost::alloc::string::String,
    #[prost(bool, tag="16")]
    pub reset_default_tblspc: bool,
    #[prost(string, tag="17")]
    pub access_method: ::prost::alloc::string::String,
    #[prost(message, optional, boxed, tag="18")]
    pub where_clause: ::core::option::Option<::prost::alloc::boxed::Box<Node>>,
    #[prost(message, optional, tag="19")]
    pub pktable: ::core::option::Option<RangeVar>,
    #[prost(message, repeated, tag="20")]
    pub fk_attrs: ::prost::alloc::vec::Vec<Node>,
    #[prost(message, repeated, tag="21")]
    pub pk_attrs: ::prost::alloc::vec::Vec<Node>,
    #[prost(string, tag="22")]
    pub fk_matchtype: ::prost::alloc::string::String,
    #[prost(string, tag="23")]
    pub fk_upd_action: ::prost::alloc::string::String,
    #[prost(string, tag="24")]
    pub fk_del_action: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="25")]
    pub old_conpfeqop: ::prost::alloc::vec::Vec<Node>,
    #[prost(uint32, tag="26")]
    pub old_pktable_oid: u32,
    #[prost(bool, tag="27")]
    pub skip_validation: bool,
    #[prost(bool, tag="28")]
    pub initially_valid: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DefElem {
    #[prost(string, tag="1")]
    pub defnamespace: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub defname: ::prost::alloc::string::String,
    #[prost(message, optional, boxed, tag="3")]
    pub arg: ::core::option::Option<::prost::alloc::boxed::Box<Node>>,
    #[prost(enumeration="DefElemAction", tag="4")]
    pub defaction: i32,
    #[prost(int32, tag="5")]
    pub location: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RangeTblEntry {
    #[prost(enumeration="RteKind", tag="1")]
    pub rtekind: i32,
    #[prost(uint32, tag="2")]
    pub relid: u32,
    #[prost(string, tag="3")]
    pub relkind: ::prost::alloc::string::String,
    #[prost(int32, tag="4")]
    pub rellockmode: i32,
    #[prost(message, optional, boxed, tag="5")]
    pub tablesample: ::core::option::Option<::prost::alloc::boxed::Box<TableSampleClause>>,
    #[prost(message, optional, boxed, tag="6")]
    pub subquery: ::core::option::Option<::prost::alloc::boxed::Box<Query>>,
    #[prost(bool, tag="7")]
    pub security_barrier: bool,
    #[prost(enumeration="JoinType", tag="8")]
    pub jointype: i32,
    #[prost(int32, tag="9")]
    pub joinmergedcols: i32,
    #[prost(message, repeated, tag="10")]
    pub joinaliasvars: ::prost::alloc::vec::Vec<Node>,
    #[prost(message, repeated, tag="11")]
    pub joinleftcols: ::prost::alloc::vec::Vec<Node>,
    #[prost(message, repeated, tag="12")]
    pub joinrightcols: ::prost::alloc::vec::Vec<Node>,
    #[prost(message, repeated, tag="13")]
    pub functions: ::prost::alloc::vec::Vec<Node>,
    #[prost(bool, tag="14")]
    pub funcordinality: bool,
    #[prost(message, optional, boxed, tag="15")]
    pub tablefunc: ::core::option::Option<::prost::alloc::boxed::Box<TableFunc>>,
    #[prost(message, repeated, tag="16")]
    pub values_lists: ::prost::alloc::vec::Vec<Node>,
    #[prost(string, tag="17")]
    pub ctename: ::prost::alloc::string::String,
    #[prost(uint32, tag="18")]
    pub ctelevelsup: u32,
    #[prost(bool, tag="19")]
    pub self_reference: bool,
    #[prost(message, repeated, tag="20")]
    pub coltypes: ::prost::alloc::vec::Vec<Node>,
    #[prost(message, repeated, tag="21")]
    pub coltypmods: ::prost::alloc::vec::Vec<Node>,
    #[prost(message, repeated, tag="22")]
    pub colcollations: ::prost::alloc::vec::Vec<Node>,
    #[prost(string, tag="23")]
    pub enrname: ::prost::alloc::string::String,
    #[prost(double, tag="24")]
    pub enrtuples: f64,
    #[prost(message, optional, tag="25")]
    pub alias: ::core::option::Option<Alias>,
    #[prost(message, optional, tag="26")]
    pub eref: ::core::option::Option<Alias>,
    #[prost(bool, tag="27")]
    pub lateral: bool,
    #[prost(bool, tag="28")]
    pub inh: bool,
    #[prost(bool, tag="29")]
    pub in_from_cl: bool,
    #[prost(uint32, tag="30")]
    pub required_perms: u32,
    #[prost(uint32, tag="31")]
    pub check_as_user: u32,
    #[prost(uint64, repeated, tag="32")]
    pub selected_cols: ::prost::alloc::vec::Vec<u64>,
    #[prost(uint64, repeated, tag="33")]
    pub inserted_cols: ::prost::alloc::vec::Vec<u64>,
    #[prost(uint64, repeated, tag="34")]
    pub updated_cols: ::prost::alloc::vec::Vec<u64>,
    #[prost(uint64, repeated, tag="35")]
    pub extra_updated_cols: ::prost::alloc::vec::Vec<u64>,
    #[prost(message, repeated, tag="36")]
    pub security_quals: ::prost::alloc::vec::Vec<Node>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RangeTblFunction {
    #[prost(message, optional, boxed, tag="1")]
    pub funcexpr: ::core::option::Option<::prost::alloc::boxed::Box<Node>>,
    #[prost(int32, tag="2")]
    pub funccolcount: i32,
    #[prost(message, repeated, tag="3")]
    pub funccolnames: ::prost::alloc::vec::Vec<Node>,
    #[prost(message, repeated, tag="4")]
    pub funccoltypes: ::prost::alloc::vec::Vec<Node>,
    #[prost(message, repeated, tag="5")]
    pub funccoltypmods: ::prost::alloc::vec::Vec<Node>,
    #[prost(message, repeated, tag="6")]
    pub funccolcollations: ::prost::alloc::vec::Vec<Node>,
    #[prost(uint64, repeated, tag="7")]
    pub funcparams: ::prost::alloc::vec::Vec<u64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TableSampleClause {
    #[prost(uint32, tag="1")]
    pub tsmhandler: u32,
    #[prost(message, repeated, tag="2")]
    pub args: ::prost::alloc::vec::Vec<Node>,
    #[prost(message, optional, boxed, tag="3")]
    pub repeatable: ::core::option::Option<::prost::alloc::boxed::Box<Node>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WithCheckOption {
    #[prost(enumeration="WcoKind", tag="1")]
    pub kind: i32,
    #[prost(string, tag="2")]
    pub relname: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub polname: ::prost::alloc::string::String,
    #[prost(message, optional, boxed, tag="4")]
    pub qual: ::core::option::Option<::prost::alloc::boxed::Box<Node>>,
    #[prost(bool, tag="5")]
    pub cascaded: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SortGroupClause {
    #[prost(uint32, tag="1")]
    pub tle_sort_group_ref: u32,
    #[prost(uint32, tag="2")]
    pub eqop: u32,
    #[prost(uint32, tag="3")]
    pub sortop: u32,
    #[prost(bool, tag="4")]
    pub nulls_first: bool,
    #[prost(bool, tag="5")]
    pub hashable: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupingSet {
    #[prost(enumeration="GroupingSetKind", tag="1")]
    pub kind: i32,
    #[prost(message, repeated, tag="2")]
    pub content: ::prost::alloc::vec::Vec<Node>,
    #[prost(int32, tag="3")]
    pub location: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WindowClause {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub refname: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="3")]
    pub partition_clause: ::prost::alloc::vec::Vec<Node>,
    #[prost(message, repeated, tag="4")]
    pub order_clause: ::prost::alloc::vec::Vec<Node>,
    #[prost(int32, tag="5")]
    pub frame_options: i32,
    #[prost(message, optional, boxed, tag="6")]
    pub start_offset: ::core::option::Option<::prost::alloc::boxed::Box<Node>>,
    #[prost(message, optional, boxed, tag="7")]
    pub end_offset: ::core::option::Option<::prost::alloc::boxed::Box<Node>>,
    #[prost(uint32, tag="8")]
    pub start_in_range_func: u32,
    #[prost(uint32, tag="9")]
    pub end_in_range_func: u32,
    #[prost(uint32, tag="10")]
    pub in_range_coll: u32,
    #[prost(bool, tag="11")]
    pub in_range_asc: bool,
    #[prost(bool, tag="12")]
    pub in_range_nulls_first: bool,
    #[prost(uint32, tag="13")]
    pub winref: u32,
    #[prost(bool, tag="14")]
    pub copied_order: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ObjectWithArgs {
    #[prost(message, repeated, tag="1")]
    pub objname: ::prost::alloc::vec::Vec<Node>,
    #[prost(message, repeated, tag="2")]
    pub objargs: ::prost::alloc::vec::Vec<Node>,
    #[prost(bool, tag="3")]
    pub args_unspecified: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccessPriv {
    #[prost(string, tag="1")]
    pub priv_name: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="2")]
    pub cols: ::prost::alloc::vec::Vec<Node>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateOpClassItem {
    #[prost(int32, tag="1")]
    pub itemtype: i32,
    #[prost(message, optional, tag="2")]
    pub name: ::core::option::Option<ObjectWithArgs>,
    #[prost(int32, tag="3")]
    pub number: i32,
    #[prost(message, repeated, tag="4")]
    pub order_family: ::prost::alloc::vec::Vec<Node>,
    #[prost(message, repeated, tag="5")]
    pub class_args: ::prost::alloc::vec::Vec<Node>,
    #[prost(message, optional, tag="6")]
    pub storedtype: ::core::option::Option<TypeName>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TableLikeClause {
    #[prost(message, optional, tag="1")]
    pub relation: ::core::option::Option<RangeVar>,
    #[prost(uint32, tag="2")]
    pub options: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FunctionParameter {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub arg_type: ::core::option::Option<TypeName>,
    #[prost(enumeration="FunctionParameterMode", tag="3")]
    pub mode: i32,
    #[prost(message, optional, boxed, tag="4")]
    pub defexpr: ::core::option::Option<::prost::alloc::boxed::Box<Node>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LockingClause {
    #[prost(message, repeated, tag="1")]
    pub locked_rels: ::prost::alloc::vec::Vec<Node>,
    #[prost(enumeration="LockClauseStrength", tag="2")]
    pub strength: i32,
    #[prost(enumeration="LockWaitPolicy", tag="3")]
    pub wait_policy: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RowMarkClause {
    #[prost(uint32, tag="1")]
    pub rti: u32,
    #[prost(enumeration="LockClauseStrength", tag="2")]
    pub strength: i32,
    #[prost(enumeration="LockWaitPolicy", tag="3")]
    pub wait_policy: i32,
    #[prost(bool, tag="4")]
    pub pushed_down: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct XmlSerialize {
    #[prost(enumeration="XmlOptionType", tag="1")]
    pub xmloption: i32,
    #[prost(message, optional, boxed, tag="2")]
    pub expr: ::core::option::Option<::prost::alloc::boxed::Box<Node>>,
    #[prost(message, optional, tag="3")]
    pub type_name: ::core::option::Option<TypeName>,
    #[prost(int32, tag="4")]
    pub location: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WithClause {
    #[prost(message, repeated, tag="1")]
    pub ctes: ::prost::alloc::vec::Vec<Node>,
    #[prost(bool, tag="2")]
    pub recursive: bool,
    #[prost(int32, tag="3")]
    pub location: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InferClause {
    #[prost(message, repeated, tag="1")]
    pub index_elems: ::prost::alloc::vec::Vec<Node>,
    #[prost(message, optional, boxed, tag="2")]
    pub where_clause: ::core::option::Option<::prost::alloc::boxed::Box<Node>>,
    #[prost(string, tag="3")]
    pub conname: ::prost::alloc::string::String,
    #[prost(int32, tag="4")]
    pub location: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OnConflictClause {
    #[prost(enumeration="OnConflictAction", tag="1")]
    pub action: i32,
    #[prost(message, optional, boxed, tag="2")]
    pub infer: ::core::option::Option<::prost::alloc::boxed::Box<InferClause>>,
    #[prost(message, repeated, tag="3")]
    pub target_list: ::prost::alloc::vec::Vec<Node>,
    #[prost(message, optional, boxed, tag="4")]
    pub where_clause: ::core::option::Option<::prost::alloc::boxed::Box<Node>>,
    #[prost(int32, tag="5")]
    pub location: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommonTableExpr {
    #[prost(string, tag="1")]
    pub ctename: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="2")]
    pub aliascolnames: ::prost::alloc::vec::Vec<Node>,
    #[prost(enumeration="CteMaterialize", tag="3")]
    pub ctematerialized: i32,
    #[prost(message, optional, boxed, tag="4")]
    pub ctequery: ::core::option::Option<::prost::alloc::boxed::Box<Node>>,
    #[prost(int32, tag="5")]
    pub location: i32,
    #[prost(bool, tag="6")]
    pub cterecursive: bool,
    #[prost(int32, tag="7")]
    pub cterefcount: i32,
    #[prost(message, repeated, tag="8")]
    pub ctecolnames: ::prost::alloc::vec::Vec<Node>,
    #[prost(message, repeated, tag="9")]
    pub ctecoltypes: ::prost::alloc::vec::Vec<Node>,
    #[prost(message, repeated, tag="10")]
    pub ctecoltypmods: ::prost::alloc::vec::Vec<Node>,
    #[prost(message, repeated, tag="11")]
    pub ctecolcollations: ::prost::alloc::vec::Vec<Node>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RoleSpec {
    #[prost(enumeration="RoleSpecType", tag="1")]
    pub roletype: i32,
    #[prost(string, tag="2")]
    pub rolename: ::prost::alloc::string::String,
    #[prost(int32, tag="3")]
    pub location: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TriggerTransition {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(bool, tag="2")]
    pub is_new: bool,
    #[prost(bool, tag="3")]
    pub is_table: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PartitionElem {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, optional, boxed, tag="2")]
    pub expr: ::core::option::Option<::prost::alloc::boxed::Box<Node>>,
    #[prost(message, repeated, tag="3")]
    pub collation: ::prost::alloc::vec::Vec<Node>,
    #[prost(message, repeated, tag="4")]
    pub opclass: ::prost::alloc::vec::Vec<Node>,
    #[prost(int32, tag="5")]
    pub location: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PartitionSpec {
    #[prost(string, tag="1")]
    pub strategy: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="2")]
    pub part_params: ::prost::alloc::vec::Vec<Node>,
    #[prost(int32, tag="3")]
    pub location: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PartitionBoundSpec {
    #[prost(string, tag="1")]
    pub strategy: ::prost::alloc::string::String,
    #[prost(bool, tag="2")]
    pub is_default: bool,
    #[prost(int32, tag="3")]
    pub modulus: i32,
    #[prost(int32, tag="4")]
    pub remainder: i32,
    #[prost(message, repeated, tag="5")]
    pub listdatums: ::prost::alloc::vec::Vec<Node>,
    #[prost(message, repeated, tag="6")]
    pub lowerdatums: ::prost::alloc::vec::Vec<Node>,
    #[prost(message, repeated, tag="7")]
    pub upperdatums: ::prost::alloc::vec::Vec<Node>,
    #[prost(int32, tag="8")]
    pub location: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PartitionRangeDatum {
    #[prost(enumeration="PartitionRangeDatumKind", tag="1")]
    pub kind: i32,
    #[prost(message, optional, boxed, tag="2")]
    pub value: ::core::option::Option<::prost::alloc::boxed::Box<Node>>,
    #[prost(int32, tag="3")]
    pub location: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PartitionCmd {
    #[prost(message, optional, tag="1")]
    pub name: ::core::option::Option<RangeVar>,
    #[prost(message, optional, tag="2")]
    pub bound: ::core::option::Option<PartitionBoundSpec>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VacuumRelation {
    #[prost(message, optional, tag="1")]
    pub relation: ::core::option::Option<RangeVar>,
    #[prost(uint32, tag="2")]
    pub oid: u32,
    #[prost(message, repeated, tag="3")]
    pub va_cols: ::prost::alloc::vec::Vec<Node>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InlineCodeBlock {
    #[prost(string, tag="1")]
    pub source_text: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub lang_oid: u32,
    #[prost(bool, tag="3")]
    pub lang_is_trusted: bool,
    #[prost(bool, tag="4")]
    pub atomic: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CallContext {
    #[prost(bool, tag="1")]
    pub atomic: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScanToken {
    #[prost(int32, tag="1")]
    pub start: i32,
    #[prost(int32, tag="2")]
    pub end: i32,
    #[prost(enumeration="Token", tag="4")]
    pub token: i32,
    #[prost(enumeration="KeywordKind", tag="5")]
    pub keyword_kind: i32,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OverridingKind {
    Undefined = 0,
    OverridingNotSet = 1,
    OverridingUserValue = 2,
    OverridingSystemValue = 3,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum QuerySource {
    Undefined = 0,
    QsrcOriginal = 1,
    QsrcParser = 2,
    QsrcInsteadRule = 3,
    QsrcQualInsteadRule = 4,
    QsrcNonInsteadRule = 5,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SortByDir {
    Undefined = 0,
    SortbyDefault = 1,
    SortbyAsc = 2,
    SortbyDesc = 3,
    SortbyUsing = 4,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SortByNulls {
    Undefined = 0,
    SortbyNullsDefault = 1,
    SortbyNullsFirst = 2,
    SortbyNullsLast = 3,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AExprKind {
    Undefined = 0,
    AexprOp = 1,
    AexprOpAny = 2,
    AexprOpAll = 3,
    AexprDistinct = 4,
    AexprNotDistinct = 5,
    AexprNullif = 6,
    AexprOf = 7,
    AexprIn = 8,
    AexprLike = 9,
    AexprIlike = 10,
    AexprSimilar = 11,
    AexprBetween = 12,
    AexprNotBetween = 13,
    AexprBetweenSym = 14,
    AexprNotBetweenSym = 15,
    AexprParen = 16,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum RoleSpecType {
    Undefined = 0,
    RolespecCstring = 1,
    RolespecCurrentUser = 2,
    RolespecSessionUser = 3,
    RolespecPublic = 4,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TableLikeOption {
    Undefined = 0,
    CreateTableLikeComments = 1,
    CreateTableLikeConstraints = 2,
    CreateTableLikeDefaults = 3,
    CreateTableLikeGenerated = 4,
    CreateTableLikeIdentity = 5,
    CreateTableLikeIndexes = 6,
    CreateTableLikeStatistics = 7,
    CreateTableLikeStorage = 8,
    CreateTableLikeAll = 9,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DefElemAction {
    Undefined = 0,
    DefelemUnspec = 1,
    DefelemSet = 2,
    DefelemAdd = 3,
    DefelemDrop = 4,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PartitionRangeDatumKind {
    Undefined = 0,
    PartitionRangeDatumMinvalue = 1,
    PartitionRangeDatumValue = 2,
    PartitionRangeDatumMaxvalue = 3,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum RteKind {
    RtekindUndefined = 0,
    RteRelation = 1,
    RteSubquery = 2,
    RteJoin = 3,
    RteFunction = 4,
    RteTablefunc = 5,
    RteValues = 6,
    RteCte = 7,
    RteNamedtuplestore = 8,
    RteResult = 9,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum WcoKind {
    WcokindUndefined = 0,
    WcoViewCheck = 1,
    WcoRlsInsertCheck = 2,
    WcoRlsUpdateCheck = 3,
    WcoRlsConflictCheck = 4,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum GroupingSetKind {
    Undefined = 0,
    GroupingSetEmpty = 1,
    GroupingSetSimple = 2,
    GroupingSetRollup = 3,
    GroupingSetCube = 4,
    GroupingSetSets = 5,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CteMaterialize {
    CtematerializeUndefined = 0,
    Default = 1,
    Always = 2,
    Never = 3,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SetOperation {
    Undefined = 0,
    SetopNone = 1,
    SetopUnion = 2,
    SetopIntersect = 3,
    SetopExcept = 4,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ObjectType {
    Undefined = 0,
    ObjectAccessMethod = 1,
    ObjectAggregate = 2,
    ObjectAmop = 3,
    ObjectAmproc = 4,
    ObjectAttribute = 5,
    ObjectCast = 6,
    ObjectColumn = 7,
    ObjectCollation = 8,
    ObjectConversion = 9,
    ObjectDatabase = 10,
    ObjectDefault = 11,
    ObjectDefacl = 12,
    ObjectDomain = 13,
    ObjectDomconstraint = 14,
    ObjectEventTrigger = 15,
    ObjectExtension = 16,
    ObjectFdw = 17,
    ObjectForeignServer = 18,
    ObjectForeignTable = 19,
    ObjectFunction = 20,
    ObjectIndex = 21,
    ObjectLanguage = 22,
    ObjectLargeobject = 23,
    ObjectMatview = 24,
    ObjectOpclass = 25,
    ObjectOperator = 26,
    ObjectOpfamily = 27,
    ObjectPolicy = 28,
    ObjectProcedure = 29,
    ObjectPublication = 30,
    ObjectPublicationRel = 31,
    ObjectRole = 32,
    ObjectRoutine = 33,
    ObjectRule = 34,
    ObjectSchema = 35,
    ObjectSequence = 36,
    ObjectSubscription = 37,
    ObjectStatisticExt = 38,
    ObjectTabconstraint = 39,
    ObjectTable = 40,
    ObjectTablespace = 41,
    ObjectTransform = 42,
    ObjectTrigger = 43,
    ObjectTsconfiguration = 44,
    ObjectTsdictionary = 45,
    ObjectTsparser = 46,
    ObjectTstemplate = 47,
    ObjectType = 48,
    ObjectUserMapping = 49,
    ObjectView = 50,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DropBehavior {
    Undefined = 0,
    DropRestrict = 1,
    DropCascade = 2,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AlterTableType {
    Undefined = 0,
    AtAddColumn = 1,
    AtAddColumnRecurse = 2,
    AtAddColumnToView = 3,
    AtColumnDefault = 4,
    AtCookedColumnDefault = 5,
    AtDropNotNull = 6,
    AtSetNotNull = 7,
    AtDropExpression = 8,
    AtCheckNotNull = 9,
    AtSetStatistics = 10,
    AtSetOptions = 11,
    AtResetOptions = 12,
    AtSetStorage = 13,
    AtDropColumn = 14,
    AtDropColumnRecurse = 15,
    AtAddIndex = 16,
    AtReAddIndex = 17,
    AtAddConstraint = 18,
    AtAddConstraintRecurse = 19,
    AtReAddConstraint = 20,
    AtReAddDomainConstraint = 21,
    AtAlterConstraint = 22,
    AtValidateConstraint = 23,
    AtValidateConstraintRecurse = 24,
    AtAddIndexConstraint = 25,
    AtDropConstraint = 26,
    AtDropConstraintRecurse = 27,
    AtReAddComment = 28,
    AtAlterColumnType = 29,
    AtAlterColumnGenericOptions = 30,
    AtChangeOwner = 31,
    AtClusterOn = 32,
    AtDropCluster = 33,
    AtSetLogged = 34,
    AtSetUnLogged = 35,
    AtDropOids = 36,
    AtSetTableSpace = 37,
    AtSetRelOptions = 38,
    AtResetRelOptions = 39,
    AtReplaceRelOptions = 40,
    AtEnableTrig = 41,
    AtEnableAlwaysTrig = 42,
    AtEnableReplicaTrig = 43,
    AtDisableTrig = 44,
    AtEnableTrigAll = 45,
    AtDisableTrigAll = 46,
    AtEnableTrigUser = 47,
    AtDisableTrigUser = 48,
    AtEnableRule = 49,
    AtEnableAlwaysRule = 50,
    AtEnableReplicaRule = 51,
    AtDisableRule = 52,
    AtAddInherit = 53,
    AtDropInherit = 54,
    AtAddOf = 55,
    AtDropOf = 56,
    AtReplicaIdentity = 57,
    AtEnableRowSecurity = 58,
    AtDisableRowSecurity = 59,
    AtForceRowSecurity = 60,
    AtNoForceRowSecurity = 61,
    AtGenericOptions = 62,
    AtAttachPartition = 63,
    AtDetachPartition = 64,
    AtAddIdentity = 65,
    AtSetIdentity = 66,
    AtDropIdentity = 67,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum GrantTargetType {
    Undefined = 0,
    AclTargetObject = 1,
    AclTargetAllInSchema = 2,
    AclTargetDefaults = 3,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum VariableSetKind {
    Undefined = 0,
    VarSetValue = 1,
    VarSetDefault = 2,
    VarSetCurrent = 3,
    VarSetMulti = 4,
    VarReset = 5,
    VarResetAll = 6,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ConstrType {
    Undefined = 0,
    ConstrNull = 1,
    ConstrNotnull = 2,
    ConstrDefault = 3,
    ConstrIdentity = 4,
    ConstrGenerated = 5,
    ConstrCheck = 6,
    ConstrPrimary = 7,
    ConstrUnique = 8,
    ConstrExclusion = 9,
    ConstrForeign = 10,
    ConstrAttrDeferrable = 11,
    ConstrAttrNotDeferrable = 12,
    ConstrAttrDeferred = 13,
    ConstrAttrImmediate = 14,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ImportForeignSchemaType {
    Undefined = 0,
    FdwImportSchemaAll = 1,
    FdwImportSchemaLimitTo = 2,
    FdwImportSchemaExcept = 3,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum RoleStmtType {
    Undefined = 0,
    RolestmtRole = 1,
    RolestmtUser = 2,
    RolestmtGroup = 3,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum FetchDirection {
    Undefined = 0,
    FetchForward = 1,
    FetchBackward = 2,
    FetchAbsolute = 3,
    FetchRelative = 4,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum FunctionParameterMode {
    Undefined = 0,
    FuncParamIn = 1,
    FuncParamOut = 2,
    FuncParamInout = 3,
    FuncParamVariadic = 4,
    FuncParamTable = 5,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TransactionStmtKind {
    Undefined = 0,
    TransStmtBegin = 1,
    TransStmtStart = 2,
    TransStmtCommit = 3,
    TransStmtRollback = 4,
    TransStmtSavepoint = 5,
    TransStmtRelease = 6,
    TransStmtRollbackTo = 7,
    TransStmtPrepare = 8,
    TransStmtCommitPrepared = 9,
    TransStmtRollbackPrepared = 10,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ViewCheckOption {
    Undefined = 0,
    NoCheckOption = 1,
    LocalCheckOption = 2,
    CascadedCheckOption = 3,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ClusterOption {
    Undefined = 0,
    CluoptRecheck = 1,
    CluoptVerbose = 2,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DiscardMode {
    Undefined = 0,
    DiscardAll = 1,
    DiscardPlans = 2,
    DiscardSequences = 3,
    DiscardTemp = 4,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ReindexObjectType {
    Undefined = 0,
    ReindexObjectIndex = 1,
    ReindexObjectTable = 2,
    ReindexObjectSchema = 3,
    ReindexObjectSystem = 4,
    ReindexObjectDatabase = 5,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AlterTsConfigType {
    AlterTsconfigTypeUndefined = 0,
    AlterTsconfigAddMapping = 1,
    AlterTsconfigAlterMappingForToken = 2,
    AlterTsconfigReplaceDict = 3,
    AlterTsconfigReplaceDictForToken = 4,
    AlterTsconfigDropMapping = 5,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AlterSubscriptionType {
    Undefined = 0,
    AlterSubscriptionOptions = 1,
    AlterSubscriptionConnection = 2,
    AlterSubscriptionPublication = 3,
    AlterSubscriptionRefresh = 4,
    AlterSubscriptionEnabled = 5,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OnCommitAction {
    Undefined = 0,
    OncommitNoop = 1,
    OncommitPreserveRows = 2,
    OncommitDeleteRows = 3,
    OncommitDrop = 4,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ParamKind {
    Undefined = 0,
    ParamExtern = 1,
    ParamExec = 2,
    ParamSublink = 3,
    ParamMultiexpr = 4,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CoercionContext {
    Undefined = 0,
    CoercionImplicit = 1,
    CoercionAssignment = 2,
    CoercionExplicit = 3,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CoercionForm {
    Undefined = 0,
    CoerceExplicitCall = 1,
    CoerceExplicitCast = 2,
    CoerceImplicitCast = 3,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum BoolExprType {
    Undefined = 0,
    AndExpr = 1,
    OrExpr = 2,
    NotExpr = 3,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SubLinkType {
    Undefined = 0,
    ExistsSublink = 1,
    AllSublink = 2,
    AnySublink = 3,
    RowcompareSublink = 4,
    ExprSublink = 5,
    MultiexprSublink = 6,
    ArraySublink = 7,
    CteSublink = 8,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum RowCompareType {
    Undefined = 0,
    RowcompareLt = 1,
    RowcompareLe = 2,
    RowcompareEq = 3,
    RowcompareGe = 4,
    RowcompareGt = 5,
    RowcompareNe = 6,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum MinMaxOp {
    Undefined = 0,
    IsGreatest = 1,
    IsLeast = 2,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SqlValueFunctionOp {
    SqlvalueFunctionOpUndefined = 0,
    SvfopCurrentDate = 1,
    SvfopCurrentTime = 2,
    SvfopCurrentTimeN = 3,
    SvfopCurrentTimestamp = 4,
    SvfopCurrentTimestampN = 5,
    SvfopLocaltime = 6,
    SvfopLocaltimeN = 7,
    SvfopLocaltimestamp = 8,
    SvfopLocaltimestampN = 9,
    SvfopCurrentRole = 10,
    SvfopCurrentUser = 11,
    SvfopUser = 12,
    SvfopSessionUser = 13,
    SvfopCurrentCatalog = 14,
    SvfopCurrentSchema = 15,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum XmlExprOp {
    Undefined = 0,
    IsXmlconcat = 1,
    IsXmlelement = 2,
    IsXmlforest = 3,
    IsXmlparse = 4,
    IsXmlpi = 5,
    IsXmlroot = 6,
    IsXmlserialize = 7,
    IsDocument = 8,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum XmlOptionType {
    Undefined = 0,
    XmloptionDocument = 1,
    XmloptionContent = 2,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum NullTestType {
    Undefined = 0,
    IsNull = 1,
    IsNotNull = 2,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum BoolTestType {
    Undefined = 0,
    IsTrue = 1,
    IsNotTrue = 2,
    IsFalse = 3,
    IsNotFalse = 4,
    IsUnknown = 5,
    IsNotUnknown = 6,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CmdType {
    Undefined = 0,
    CmdUnknown = 1,
    CmdSelect = 2,
    CmdUpdate = 3,
    CmdInsert = 4,
    CmdDelete = 5,
    CmdUtility = 6,
    CmdNothing = 7,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum JoinType {
    Undefined = 0,
    JoinInner = 1,
    JoinLeft = 2,
    JoinFull = 3,
    JoinRight = 4,
    JoinSemi = 5,
    JoinAnti = 6,
    JoinUniqueOuter = 7,
    JoinUniqueInner = 8,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AggStrategy {
    Undefined = 0,
    AggPlain = 1,
    AggSorted = 2,
    AggHashed = 3,
    AggMixed = 4,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AggSplit {
    Undefined = 0,
    AggsplitSimple = 1,
    AggsplitInitialSerial = 2,
    AggsplitFinalDeserial = 3,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SetOpCmd {
    Undefined = 0,
    SetopcmdIntersect = 1,
    SetopcmdIntersectAll = 2,
    SetopcmdExcept = 3,
    SetopcmdExceptAll = 4,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SetOpStrategy {
    Undefined = 0,
    SetopSorted = 1,
    SetopHashed = 2,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OnConflictAction {
    Undefined = 0,
    OnconflictNone = 1,
    OnconflictNothing = 2,
    OnconflictUpdate = 3,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum LimitOption {
    Undefined = 0,
    Default = 1,
    Count = 2,
    WithTies = 3,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum LockClauseStrength {
    Undefined = 0,
    LcsNone = 1,
    LcsForkeyshare = 2,
    LcsForshare = 3,
    LcsFornokeyupdate = 4,
    LcsForupdate = 5,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum LockWaitPolicy {
    Undefined = 0,
    LockWaitBlock = 1,
    LockWaitSkip = 2,
    LockWaitError = 3,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum LockTupleMode {
    Undefined = 0,
    LockTupleKeyShare = 1,
    LockTupleShare = 2,
    LockTupleNoKeyExclusive = 3,
    LockTupleExclusive = 4,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum KeywordKind {
    NoKeyword = 0,
    UnreservedKeyword = 1,
    ColNameKeyword = 2,
    TypeFuncNameKeyword = 3,
    ReservedKeyword = 4,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Token {
    Nul = 0,
    /// Single-character tokens that are returned 1:1 (identical with "self" list in scan.l)
    /// Either supporting syntax, or single-character operators (some can be both)
    /// Also see https://www.postgresql.org/docs/12/sql-syntax-lexical.html#SQL-SYNTAX-SPECIAL-CHARS
    ///
    /// "%"
    Ascii37 = 37,
    /// "("
    Ascii40 = 40,
    /// ")"
    Ascii41 = 41,
    /// "*"
    Ascii42 = 42,
    /// "+"
    Ascii43 = 43,
    /// ","
    Ascii44 = 44,
    /// "-"
    Ascii45 = 45,
    /// "."
    Ascii46 = 46,
    /// "/"
    Ascii47 = 47,
    /// ":"
    Ascii58 = 58,
    /// ";"
    Ascii59 = 59,
    /// "<"
    Ascii60 = 60,
    /// "="
    Ascii61 = 61,
    /// ">"
    Ascii62 = 62,
    /// "?"
    Ascii63 = 63,
    /// "["
    Ascii91 = 91,
    /// "]"
    Ascii93 = 93,
    /// "^"
    Ascii94 = 94,
    /// Named tokens in scan.l
    Ident = 258,
    Uident = 259,
    Fconst = 260,
    Sconst = 261,
    Usconst = 262,
    Bconst = 263,
    Xconst = 264,
    Op = 265,
    Iconst = 266,
    Param = 267,
    Typecast = 268,
    DotDot = 269,
    ColonEquals = 270,
    EqualsGreater = 271,
    LessEquals = 272,
    GreaterEquals = 273,
    NotEquals = 274,
    SqlComment = 275,
    CComment = 276,
    AbortP = 277,
    AbsoluteP = 278,
    Access = 279,
    Action = 280,
    AddP = 281,
    Admin = 282,
    After = 283,
    Aggregate = 284,
    All = 285,
    Also = 286,
    Alter = 287,
    Always = 288,
    Analyse = 289,
    Analyze = 290,
    And = 291,
    Any = 292,
    Array = 293,
    As = 294,
    Asc = 295,
    Assertion = 296,
    Assignment = 297,
    Asymmetric = 298,
    At = 299,
    Attach = 300,
    Attribute = 301,
    Authorization = 302,
    Backward = 303,
    Before = 304,
    BeginP = 305,
    Between = 306,
    Bigint = 307,
    Binary = 308,
    Bit = 309,
    BooleanP = 310,
    Both = 311,
    By = 312,
    Cache = 313,
    Call = 314,
    Called = 315,
    Cascade = 316,
    Cascaded = 317,
    Case = 318,
    Cast = 319,
    CatalogP = 320,
    Chain = 321,
    CharP = 322,
    Character = 323,
    Characteristics = 324,
    Check = 325,
    Checkpoint = 326,
    Class = 327,
    Close = 328,
    Cluster = 329,
    Coalesce = 330,
    Collate = 331,
    Collation = 332,
    Column = 333,
    Columns = 334,
    Comment = 335,
    Comments = 336,
    Commit = 337,
    Committed = 338,
    Concurrently = 339,
    Configuration = 340,
    Conflict = 341,
    Connection = 342,
    Constraint = 343,
    Constraints = 344,
    ContentP = 345,
    ContinueP = 346,
    ConversionP = 347,
    Copy = 348,
    Cost = 349,
    Create = 350,
    Cross = 351,
    Csv = 352,
    Cube = 353,
    CurrentP = 354,
    CurrentCatalog = 355,
    CurrentDate = 356,
    CurrentRole = 357,
    CurrentSchema = 358,
    CurrentTime = 359,
    CurrentTimestamp = 360,
    CurrentUser = 361,
    Cursor = 362,
    Cycle = 363,
    DataP = 364,
    Database = 365,
    DayP = 366,
    Deallocate = 367,
    Dec = 368,
    DecimalP = 369,
    Declare = 370,
    Default = 371,
    Defaults = 372,
    Deferrable = 373,
    Deferred = 374,
    Definer = 375,
    DeleteP = 376,
    Delimiter = 377,
    Delimiters = 378,
    Depends = 379,
    Desc = 380,
    Detach = 381,
    Dictionary = 382,
    DisableP = 383,
    Discard = 384,
    Distinct = 385,
    Do = 386,
    DocumentP = 387,
    DomainP = 388,
    DoubleP = 389,
    Drop = 390,
    Each = 391,
    Else = 392,
    EnableP = 393,
    Encoding = 394,
    Encrypted = 395,
    EndP = 396,
    EnumP = 397,
    Escape = 398,
    Event = 399,
    Except = 400,
    Exclude = 401,
    Excluding = 402,
    Exclusive = 403,
    Execute = 404,
    Exists = 405,
    Explain = 406,
    Expression = 407,
    Extension = 408,
    External = 409,
    Extract = 410,
    FalseP = 411,
    Family = 412,
    Fetch = 413,
    Filter = 414,
    FirstP = 415,
    FloatP = 416,
    Following = 417,
    For = 418,
    Force = 419,
    Foreign = 420,
    Forward = 421,
    Freeze = 422,
    From = 423,
    Full = 424,
    Function = 425,
    Functions = 426,
    Generated = 427,
    Global = 428,
    Grant = 429,
    Granted = 430,
    Greatest = 431,
    GroupP = 432,
    Grouping = 433,
    Groups = 434,
    Handler = 435,
    Having = 436,
    HeaderP = 437,
    Hold = 438,
    HourP = 439,
    IdentityP = 440,
    IfP = 441,
    Ilike = 442,
    Immediate = 443,
    Immutable = 444,
    ImplicitP = 445,
    ImportP = 446,
    InP = 447,
    Include = 448,
    Including = 449,
    Increment = 450,
    Index = 451,
    Indexes = 452,
    Inherit = 453,
    Inherits = 454,
    Initially = 455,
    InlineP = 456,
    InnerP = 457,
    Inout = 458,
    InputP = 459,
    Insensitive = 460,
    Insert = 461,
    Instead = 462,
    IntP = 463,
    Integer = 464,
    Intersect = 465,
    Interval = 466,
    Into = 467,
    Invoker = 468,
    Is = 469,
    Isnull = 470,
    Isolation = 471,
    Join = 472,
    Key = 473,
    Label = 474,
    Language = 475,
    LargeP = 476,
    LastP = 477,
    LateralP = 478,
    Leading = 479,
    Leakproof = 480,
    Least = 481,
    Left = 482,
    Level = 483,
    Like = 484,
    Limit = 485,
    Listen = 486,
    Load = 487,
    Local = 488,
    Localtime = 489,
    Localtimestamp = 490,
    Location = 491,
    LockP = 492,
    Locked = 493,
    Logged = 494,
    Mapping = 495,
    Match = 496,
    Materialized = 497,
    Maxvalue = 498,
    Method = 499,
    MinuteP = 500,
    Minvalue = 501,
    Mode = 502,
    MonthP = 503,
    Move = 504,
    NameP = 505,
    Names = 506,
    National = 507,
    Natural = 508,
    Nchar = 509,
    New = 510,
    Next = 511,
    Nfc = 512,
    Nfd = 513,
    Nfkc = 514,
    Nfkd = 515,
    No = 516,
    None = 517,
    Normalize = 518,
    Normalized = 519,
    Not = 520,
    Nothing = 521,
    Notify = 522,
    Notnull = 523,
    Nowait = 524,
    NullP = 525,
    Nullif = 526,
    NullsP = 527,
    Numeric = 528,
    ObjectP = 529,
    Of = 530,
    Off = 531,
    Offset = 532,
    Oids = 533,
    Old = 534,
    On = 535,
    Only = 536,
    Operator = 537,
    Option = 538,
    Options = 539,
    Or = 540,
    Order = 541,
    Ordinality = 542,
    Others = 543,
    OutP = 544,
    OuterP = 545,
    Over = 546,
    Overlaps = 547,
    Overlay = 548,
    Overriding = 549,
    Owned = 550,
    Owner = 551,
    Parallel = 552,
    Parser = 553,
    Partial = 554,
    Partition = 555,
    Passing = 556,
    Password = 557,
    Placing = 558,
    Plans = 559,
    Policy = 560,
    Position = 561,
    Preceding = 562,
    Precision = 563,
    Preserve = 564,
    Prepare = 565,
    Prepared = 566,
    Primary = 567,
    Prior = 568,
    Privileges = 569,
    Procedural = 570,
    Procedure = 571,
    Procedures = 572,
    Program = 573,
    Publication = 574,
    Quote = 575,
    Range = 576,
    Read = 577,
    Real = 578,
    Reassign = 579,
    Recheck = 580,
    Recursive = 581,
    Ref = 582,
    References = 583,
    Referencing = 584,
    Refresh = 585,
    Reindex = 586,
    RelativeP = 587,
    Release = 588,
    Rename = 589,
    Repeatable = 590,
    Replace = 591,
    Replica = 592,
    Reset = 593,
    Restart = 594,
    Restrict = 595,
    Returning = 596,
    Returns = 597,
    Revoke = 598,
    Right = 599,
    Role = 600,
    Rollback = 601,
    Rollup = 602,
    Routine = 603,
    Routines = 604,
    Row = 605,
    Rows = 606,
    Rule = 607,
    Savepoint = 608,
    Schema = 609,
    Schemas = 610,
    Scroll = 611,
    Search = 612,
    SecondP = 613,
    Security = 614,
    Select = 615,
    Sequence = 616,
    Sequences = 617,
    Serializable = 618,
    Server = 619,
    Session = 620,
    SessionUser = 621,
    Set = 622,
    Sets = 623,
    Setof = 624,
    Share = 625,
    Show = 626,
    Similar = 627,
    Simple = 628,
    Skip = 629,
    Smallint = 630,
    Snapshot = 631,
    Some = 632,
    SqlP = 633,
    Stable = 634,
    StandaloneP = 635,
    Start = 636,
    Statement = 637,
    Statistics = 638,
    Stdin = 639,
    Stdout = 640,
    Storage = 641,
    Stored = 642,
    StrictP = 643,
    StripP = 644,
    Subscription = 645,
    Substring = 646,
    Support = 647,
    Symmetric = 648,
    Sysid = 649,
    SystemP = 650,
    Table = 651,
    Tables = 652,
    Tablesample = 653,
    Tablespace = 654,
    Temp = 655,
    Template = 656,
    Temporary = 657,
    TextP = 658,
    Then = 659,
    Ties = 660,
    Time = 661,
    Timestamp = 662,
    To = 663,
    Trailing = 664,
    Transaction = 665,
    Transform = 666,
    Treat = 667,
    Trigger = 668,
    Trim = 669,
    TrueP = 670,
    Truncate = 671,
    Trusted = 672,
    TypeP = 673,
    TypesP = 674,
    Uescape = 675,
    Unbounded = 676,
    Uncommitted = 677,
    Unencrypted = 678,
    Union = 679,
    Unique = 680,
    Unknown = 681,
    Unlisten = 682,
    Unlogged = 683,
    Until = 684,
    Update = 685,
    User = 686,
    Using = 687,
    Vacuum = 688,
    Valid = 689,
    Validate = 690,
    Validator = 691,
    ValueP = 692,
    Values = 693,
    Varchar = 694,
    Variadic = 695,
    Varying = 696,
    Verbose = 697,
    VersionP = 698,
    View = 699,
    Views = 700,
    Volatile = 701,
    When = 702,
    Where = 703,
    WhitespaceP = 704,
    Window = 705,
    With = 706,
    Within = 707,
    Without = 708,
    Work = 709,
    Wrapper = 710,
    Write = 711,
    XmlP = 712,
    Xmlattributes = 713,
    Xmlconcat = 714,
    Xmlelement = 715,
    Xmlexists = 716,
    Xmlforest = 717,
    Xmlnamespaces = 718,
    Xmlparse = 719,
    Xmlpi = 720,
    Xmlroot = 721,
    Xmlserialize = 722,
    Xmltable = 723,
    YearP = 724,
    YesP = 725,
    Zone = 726,
    NotLa = 727,
    NullsLa = 728,
    WithLa = 729,
    Postfixop = 730,
    Uminus = 731,
}
