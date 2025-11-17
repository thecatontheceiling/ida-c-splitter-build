use crate::signature_parser::parse_signature;

#[test]
fn test_signature_001() {
    let result = parse_signature("void __fastcall std::bad_alloc::bad_alloc");
    assert_eq!(result, vec![
        "std",
        "bad_alloc",
        "bad_alloc"
    ]);
}

#[test]
fn test_signature_002() {
    let result = parse_signature("int *__fastcall std::_Uninitialized_copy<int *,int *,std::allocator<int>>");
    assert_eq!(result, vec![
        "std",
        "_Uninitialized_copy<int *,int *,std::allocator<int>>"
    ]);
}

#[test]
fn test_signature_003() {
    let result = parse_signature("void __fastcall CFileUtil::StringToUpper");
    assert_eq!(result, vec![
        "CFileUtil",
        "StringToUpper"
    ]);
}

#[test]
fn test_signature_004() {
    let result = parse_signature("boost::container::container_detail::vec_iterator<boost::container::container_detail::pair<ava::idstring<PropertyContainerIdStringTag,1>,TVariant<std::allocator<int> > > *,1> *__fastcall boost::container::container_detail::flat_tree<ava::idstring<PropertyContainerIdStringTag,1>,boost::container::container_detail::pair<ava::idstring<PropertyContainerIdStringTag,1>,TVariant<std::allocator<int>>>,boost::container::container_detail::select1st<boost::container::container_detail::pair<ava::idstring<PropertyContainerIdStringTag,1>,TVariant<std::allocator<int>>>>,std::less<ava::idstring<PropertyContainerIdStringTag,1>>,std::allocator<boost::container::container_detail::pair<ava::idstring<PropertyContainerIdStringTag,1>,TVariant<std::allocator<int>>>>>::cend");
    assert_eq!(result, vec![
        "boost",
        "container",
        "container_detail",
        "flat_tree<ava::idstring<PropertyContainerIdStringTag,1>,boost::container::container_detail::pair<ava::idstring<PropertyContainerIdStringTag,1>,TVariant<std::allocator<int>>>,boost::container::container_detail::select1st<boost::container::container_detail::pair<ava::idstring<PropertyContainerIdStringTag,1>,TVariant<std::allocator<int>>>>,std::less<ava::idstring<PropertyContainerIdStringTag,1>>,std::allocator<boost::container::container_detail::pair<ava::idstring<PropertyContainerIdStringTag,1>,TVariant<std::allocator<int>>>>>",
        "cend"
    ]);
}

#[test]
fn test_signature_005() {
    let result = parse_signature("void __fastcall signals_slots_moving_Test::signals_slots_moving_Test");
    assert_eq!(result, vec![
        "signals_slots_moving_Test",
        "signals_slots_moving_Test"
    ]);
}

#[test]
fn test_signature_006() {
    let result = parse_signature("boost::container::container_detail::vec_iterator<boost::container::container_detail::pair<ava::idstring<PropertyContainerIdStringTag,1>,TVariant<std::allocator<int> > > *,0> *__fastcall std::forward<boost::container::container_detail::vec_iterator<boost::container::container_detail::pair<ava::idstring<PropertyContainerIdStringTag,1>,TVariant<std::allocator<int>>> *,0> &>");
    assert_eq!(result, vec![
        "std",
        "forward<boost::container::container_detail::vec_iterator<boost::container::container_detail::pair<ava::idstring<PropertyContainerIdStringTag,1>,TVariant<std::allocator<int>>> *,0> &>"
    ]);
}

#[test]
fn test_signature_007() {
    let result = parse_signature("void __fastcall std::_Tree_val<std::_Tmap_traits<std::string,std::string,std::less<std::string>,std::allocator<std::pair<std::string const,std::string>>,0>>::_Tree_val<std::_Tmap_traits<std::string,std::string,std::less<std::string>,std::allocator<std::pair<std::string const,std::string>>,0>>");
    assert_eq!(result, vec![
        "std",
        "_Tree_val<std::_Tmap_traits<std::string,std::string,std::less<std::string>,std::allocator<std::pair<std::string const,std::string>>,0>>",
        "_Tree_val<std::_Tmap_traits<std::string,std::string,std::less<std::string>,std::allocator<std::pair<std::string const,std::string>>,0>>"
    ]);
}

#[test]
fn test_signature_008() {
    let result = parse_signature("char __fastcall CBmpFile::Save");
    assert_eq!(result, vec![
        "CBmpFile",
        "Save"
    ]);
}

#[test]
fn test_signature_009() {
    let result = parse_signature("std::_Vector_const_iterator<std::_Vector_val<unsigned __int64> > *__fastcall std::vector<unsigned __int64>::begin");
    assert_eq!(result, vec![
        "std",
        "vector<unsigned __int64>",
        "begin"
    ]);
}

#[test]
fn test_signature_010() {
    let result = parse_signature("void __fastcall NGraphicsEngine::CGISolver::SetGITextures");
    assert_eq!(result, vec![
        "NGraphicsEngine",
        "CGISolver",
        "SetGITextures"
    ]);
}

#[test]
fn test_signature_011() {
    let result = parse_signature("NGraphicsEngine::SGenericVertex **__fastcall std::_Move<NGraphicsEngine::SGenericVertex * &>");
    assert_eq!(result, vec![
        "std",
        "_Move<NGraphicsEngine::SGenericVertex * &>"
    ]);
}

#[test]
fn test_signature_012() {
    let result = parse_signature("std::_Vector_iterator<std::_Vector_val<TResourceCachePtr<SEnvironmentParameters>> > *__fastcall std::vector<TResourceCachePtr<SEnvironmentParameters>>::end");
    assert_eq!(result, vec![
        "std",
        "vector<TResourceCachePtr<SEnvironmentParameters>>",
        "end"
    ]);
}

#[test]
fn test_signature_013() {
    let result = parse_signature("std::_Vector_iterator<std::_Vector_val<NGraphicsEngine::CFontManager::SFontEntry> > *__fastcall std::vector<NGraphicsEngine::CFontManager::SFontEntry>::emplace<NGraphicsEngine::CFontManager::SFontEntry &>");
    assert_eq!(result, vec![
        "std",
        "vector<NGraphicsEngine::CFontManager::SFontEntry>",
        "emplace<NGraphicsEngine::CFontManager::SFontEntry &>"
    ]);
}

#[test]
fn test_signature_014() {
    let result = parse_signature("unsigned __int64 __fastcall NGraphicsEngine::CRenderBlockGeneralJC3::GetSortID");
    assert_eq!(result, vec![
        "NGraphicsEngine",
        "CRenderBlockGeneralJC3",
        "GetSortID"
    ]);
}

#[test]
fn test_signature_015() {
    let result = parse_signature("void __fastcall NGraphicsEngine::CRenderBlockTrail::CRenderBlockTrail");
    assert_eq!(result, vec![
        "NGraphicsEngine",
        "CRenderBlockTrail",
        "CRenderBlockTrail"
    ]);
}

#[test]
fn test_signature_016() {
    let result = parse_signature("void __fastcall NGraphicsEngine::CPostEffectsManager::SetU2LinearAngle");
    assert_eq!(result, vec![
        "NGraphicsEngine",
        "CPostEffectsManager",
        "SetU2LinearAngle"
    ]);
}

#[test]
fn test_signature_017() {
    let result = parse_signature("char __fastcall NGraphicsEngine::CRenderBlockCharacterSkin::IsOpaque");
    assert_eq!(result, vec![
        "NGraphicsEngine",
        "CRenderBlockCharacterSkin",
        "IsOpaque"
    ]);
}

#[test]
fn test_signature_018() {
    let result = parse_signature("std::_Vector_iterator<std::_Vector_val<NGraphicsEngine::CRenderPass *> > *__fastcall std::_Vector_iterator<std::_Vector_val<NGraphicsEngine::CRenderPass *>>::_Rechecked");
    assert_eq!(result, vec![
        "std",
        "_Vector_iterator<std::_Vector_val<NGraphicsEngine::CRenderPass *>>",
        "_Rechecked"
    ]);
}

#[test]
fn test_signature_019() {
    let result = parse_signature("void __fastcall std::_Make_heap<float *,__int64,float>");
    assert_eq!(result, vec![
        "std",
        "_Make_heap<float *,__int64,float>"
    ]);
}

#[test]
fn test_signature_020() {
    let result = parse_signature("_BOOL8 __fastcall IPfxInstance::IsModifyManifoldActive");
    assert_eq!(result, vec![
        "IPfxInstance",
        "IsModifyManifoldActive"
    ]);
}

#[test]
fn test_signature_021() {
    let result = parse_signature("hknpShapeManager::MutableShapeInfo **__fastcall hkArrayBase<hknpShapeManager::MutableShapeInfo *>::operator[]");
    assert_eq!(result, vec![
        "hkArrayBase<hknpShapeManager::MutableShapeInfo *>",
        "operator[]"
    ]);
}

#[test]
fn test_signature_022() {
    let result = parse_signature("void *__fastcall CustomBreakOffModifier::operator new");
    assert_eq!(result, vec![
        "CustomBreakOffModifier",
        "operator new"
    ]);
}

#[test]
fn test_signature_023() {
    let result = parse_signature("void __fastcall hkndBreakableBody::setHierarchyInstance");
    assert_eq!(result, vec![
        "hkndBreakableBody",
        "setHierarchyInstance"
    ]);
}

#[test]
fn test_signature_024() {
    let result = parse_signature("void __fastcall hkArray<hkMassElement,hkContainerHeapAllocator>::~hkArray<hkMassElement,hkContainerHeapAllocator>");
    assert_eq!(result, vec![
        "hkArray<hkMassElement,hkContainerHeapAllocator>",
        "~hkArray<hkMassElement,hkContainerHeapAllocator>"
    ]);
}

#[test]
fn test_signature_025() {
    let result = parse_signature("const char *__fastcall hknpMorphExternMesh::getTypeIdentifier");
    assert_eq!(result, vec![
        "hknpMorphExternMesh",
        "getTypeIdentifier"
    ]);
}

#[test]
fn test_signature_026() {
    let result = parse_signature("void __fastcall NPfxBreakable::ForEachValidPart_Reversed<`anonymous namespace'::_lambda7_>");
    assert_eq!(result, vec![
        "NPfxBreakable",
        "ForEachValidPart_Reversed<`anonymous namespace'::_lambda7_>"
    ]);
}

#[test]
fn test_signature_027() {
    let result = parse_signature("hknpShapeTagCodec *__fastcall hknpShapeTagCodec::`vector deleting destructor'");
    assert_eq!(result, vec![
        "hknpShapeTagCodec",
        "`vector deleting destructor'"
    ]);
}

#[test]
fn test_signature_028() {
    let result = parse_signature("void __fastcall hkSignal2<hknpEventHandlerInput const &,hknpEvent const &>::MemberSlot<CRigidBodyCharacterCollisionListener,void (CRigidBodyCharacterCollisionListener::*)(hknpEventHandlerInput const &,hknpEvent const &)>::operator delete");
    assert_eq!(result, vec![
        "hkSignal2<hknpEventHandlerInput const &,hknpEvent const &>",
        "MemberSlot<CRigidBodyCharacterCollisionListener,void (CRigidBodyCharacterCollisionListener::*)(hknpEventHandlerInput const &,hknpEvent const &)>",
        "operator delete"
    ]);
}

#[test]
fn test_signature_029() {
    let result = parse_signature("std::tr1::_Impl_no_alloc0<std::tr1::_Callable_obj<NPfxBreakable::SDeferredImpulseAtPointApplicator_World<CPfxBreakable,2887347789>,0>,void> *__fastcall std::tr1::_Impl_no_alloc0<std::tr1::_Callable_obj<NPfxBreakable::SDeferredImpulseAtPointApplicator_World<CPfxBreakable,2887347789>,0>,void>::`scalar deleting destructor'");
    assert_eq!(result, vec![
        "std",
        "tr1",
        "_Impl_no_alloc0<std::tr1::_Callable_obj<NPfxBreakable::SDeferredImpulseAtPointApplicator_World<CPfxBreakable,2887347789>,0>,void>",
        "`scalar deleting destructor'"
    ]);
}

#[test]
fn test_signature_030() {
    let result = parse_signature("void __fastcall hkArray<hkBlockStreamBase::LinkedRange,hkContainerHeapAllocator>::clearAndDeallocate");
    assert_eq!(result, vec![
        "hkArray<hkBlockStreamBase::LinkedRange,hkContainerHeapAllocator>",
        "clearAndDeallocate"
    ]);
}

#[test]
fn test_signature_031() {
    let result = parse_signature("std::back_insert_iterator<std::vector<int> > *__fastcall std::_Merge<int *,int *,std::back_insert_iterator<std::vector<int>>,std::less<int>>");
    assert_eq!(result, vec![
        "std",
        "_Merge<int *,int *,std::back_insert_iterator<std::vector<int>>,std::less<int>>"
    ]);
}

#[test]
fn test_signature_032() {
    let result = parse_signature("unsigned __int64 __fastcall std::vector<Input::SDeviceObjectInfo>::max_size");
    assert_eq!(result, vec![
        "std",
        "vector<Input::SDeviceObjectInfo>",
        "max_size"
    ]);
}

#[test]
fn test_signature_033() {
    let result = parse_signature("boost::container::container_detail::vec_iterator<boost::container::container_detail::pair<ava::idstring<PropertyContainerIdStringTag,1>,TPropertyContainer<ava::idstring<PropertyContainerIdStringTag,1>,std::allocator<int>,CIDStringHash> > *,0> *__fastcall std::move<boost::container::container_detail::vec_iterator<boost::container::container_detail::pair<ava::idstring<PropertyContainerIdStringTag,1>,TPropertyContainer<ava::idstring<PropertyContainerIdStringTag,1>,std::allocator<int>,CIDStringHash>> *,0> &>");
    assert_eq!(result, vec![
        "std",
        "move<boost::container::container_detail::vec_iterator<boost::container::container_detail::pair<ava::idstring<PropertyContainerIdStringTag,1>,TPropertyContainer<ava::idstring<PropertyContainerIdStringTag,1>,std::allocator<int>,CIDStringHash>> *,0> &>"
    ]);
}

#[test]
fn test_signature_034() {
    let result = parse_signature("unsigned __int64 __fastcall _End___THashTable_USTextureFactor__1__Create_CMeshModel_NModelSystem__SA_AV__shared_ptr_VCMeshModel_NModelSystem___boost__AEBVCHashString__PEAPEAVIRenderBlock_NGraphicsEngine__I_Z_I_00G__QEBA_KXZ");
    assert_eq!(result, vec![
        "_End___THashTable_USTextureFactor__1__Create_CMeshModel_NModelSystem__SA_AV__shared_ptr_VCMeshModel_NModelSystem___boost__AEBVCHashString__PEAPEAVIRenderBlock_NGraphicsEngine__I_Z_I_00G__QEBA_KXZ"
    ]);
}

#[test]
fn test_signature_035() {
    let result = parse_signature("CSkyGradient *__fastcall CAtmosphere::AccessSky");
    assert_eq!(result, vec![
        "CAtmosphere",
        "AccessSky"
    ]);
}

#[test]
fn test_signature_036() {
    let result = parse_signature("void __fastcall std::allocator<NWater::SWaterBox *>::allocator<NWater::SWaterBox *>");
    assert_eq!(result, vec![
        "std",
        "allocator<NWater::SWaterBox *>",
        "allocator<NWater::SWaterBox *>"
    ]);
}

#[test]
fn test_signature_037() {
    let result = parse_signature("void __fastcall TArray<CLandscapeEffects::SRenderPreset>::SetSize");
    assert_eq!(result, vec![
        "TArray<CLandscapeEffects::SRenderPreset>",
        "SetSize"
    ]);
}

#[test]
fn test_signature_038() {
    let result = parse_signature("std::tr1::_Impl_base1<void,float const *> *__fastcall std::tr1::_Impl_no_alloc1<std::tr1::_Callable_obj<NGraphicsEngine::`anonymous namespace'::_lambda23_,0>,void,float const *>::_Copy");
    assert_eq!(result, vec![
        "std",
        "tr1",
        "_Impl_no_alloc1<std::tr1::_Callable_obj<NGraphicsEngine::`anonymous namespace'::_lambda23_,0>,void,float const *>",
        "_Copy"
    ]);
}

#[test]
fn test_signature_039() {
    let result = parse_signature("bool __fastcall NGraphicsEngine::CNVWaterScheduler::isWorkDone");
    assert_eq!(result, vec![
        "NGraphicsEngine",
        "CNVWaterScheduler",
        "isWorkDone"
    ]);
}

#[test]
fn test_signature_040() {
    let result = parse_signature("void __fastcall NGraphicsEngine::CRenderBlockOpen::SetColorMultiplier");
    assert_eq!(result, vec![
        "NGraphicsEngine",
        "CRenderBlockOpen",
        "SetColorMultiplier"
    ]);
}

#[test]
fn test_signature_041() {
    let result = parse_signature("unsigned __int64 __fastcall iparray<index_ptr<NAnimationSystem::CBoxBase>,16>::size");
    assert_eq!(result, vec![
        "iparray<index_ptr<NAnimationSystem::CBoxBase>,16>",
        "size"
    ]);
}

#[test]
fn test_signature_042() {
    let result = parse_signature("boost::container::container_detail::vec_iterator<std::pair<CHashString,ava::idstring<NAnimationSystem::SAnimationIdTag,0> > *,1> *__fastcall boost::container::container_detail::force_copy_boost::container::container_detail::vec_iterator_std::pair_CHashString_ava::idstring_NAnimationSystem::SAnimationIdTag_0________ptr64_1__boost::container::container_detail::vec_iterator_boost::container::container_detail::pair_CHashString_ava::idstring_NAnimationSystem::SAnimationIdTag_0________ptr64_1___");
    assert_eq!(result, vec![
        "boost",
        "container",
        "container_detail",
        "force_copy_boost",
        "container",
        "container_detail",
        "vec_iterator_std",
        "pair_CHashString_ava",
        "idstring_NAnimationSystem",
        "SAnimationIdTag_0________ptr64_1__boost",
        "container",
        "container_detail",
        "vec_iterator_boost",
        "container",
        "container_detail",
        "pair_CHashString_ava",
        "idstring_NAnimationSystem",
        "SAnimationIdTag_0________ptr64_1___"
    ]);
}

#[test]
fn test_signature_043() {
    let result = parse_signature("NAnimationSystem::CPlug *__fastcall NAnimationSystem::CPlug::`scalar deleting destructor'");
    assert_eq!(result, vec![
        "NAnimationSystem",
        "CPlug",
        "`scalar deleting destructor'"
    ]);
}

#[test]
fn test_signature_044() {
    let result = parse_signature("NAnimationSystem::CBinaryBoxBase<NAnimationSystem::CIsDifferentBox,float,float,float> *__fastcall NAnimationSystem::CBinaryBoxBase<NAnimationSystem::CIsDifferentBox,float,float,float>::`scalar deleting destructor'");
    assert_eq!(result, vec![
        "NAnimationSystem",
        "CBinaryBoxBase<NAnimationSystem::CIsDifferentBox,float,float,float>",
        "`scalar deleting destructor'"
    ]);
}

#[test]
fn test_signature_045() {
    let result = parse_signature("void __fastcall std::_Cons_val<std::allocator<NAnimationSystem::CHumanIK::HIKEffectorTargetPosition>,NAnimationSystem::CHumanIK::HIKEffectorTargetPosition,NAnimationSystem::CHumanIK::HIKEffectorTargetPosition const &>");
    assert_eq!(result, vec![
        "std",
        "_Cons_val<std::allocator<NAnimationSystem::CHumanIK::HIKEffectorTargetPosition>,NAnimationSystem::CHumanIK::HIKEffectorTargetPosition,NAnimationSystem::CHumanIK::HIKEffectorTargetPosition const &>"
    ]);
}

#[test]
fn test_signature_046() {
    let result = parse_signature("std::_Tree_iterator<std::_Tree_val<std::_Tmap_traits<std::string,NAnimationSystem::CCreatorBase<NAnimationSystem::CBoxBase> *,std::less<std::string >,std::allocator<std::pair<std::string const ,NAnimationSystem::CCreatorBase<NAnimationSystem::CBoxBase> *> >,0> > > *__fastcall std::_Tree<std::_Tmap_traits<std::string,NAnimationSystem::CCreatorBase<NAnimationSystem::CBoxBase> *,std::less<std::string>,std::allocator<std::pair<std::string const,NAnimationSystem::CCreatorBase<NAnimationSystem::CBoxBase> *>>,0>>::erase");
    assert_eq!(result, vec![
        "std",
        "_Tree<std::_Tmap_traits<std::string,NAnimationSystem::CCreatorBase<NAnimationSystem::CBoxBase> *,std::less<std::string>,std::allocator<std::pair<std::string const,NAnimationSystem::CCreatorBase<NAnimationSystem::CBoxBase> *>>,0>>",
        "erase"
    ]);
}

#[test]
fn test_signature_047() {
    let result = parse_signature("void __fastcall SCameraId::SetDebugName");
    assert_eq!(result, vec![
        "SCameraId",
        "SetDebugName"
    ]);
}

#[test]
fn test_signature_048() {
    let result = parse_signature("void __fastcall std::_Tmap_traits<void *,CVector3f,std::less<void *>,std::allocator<std::pair<void * const,CVector3f>>,0>::_Tmap_traits<void *,CVector3f,std::less<void *>,std::allocator<std::pair<void * const,CVector3f>>,0>");
    assert_eq!(result, vec![
        "std",
        "_Tmap_traits<void *,CVector3f,std::less<void *>,std::allocator<std::pair<void * const,CVector3f>>,0>",
        "_Tmap_traits<void *,CVector3f,std::less<void *>,std::allocator<std::pair<void * const,CVector3f>>,0>"
    ]);
}

#[test]
fn test_signature_049() {
    let result = parse_signature("std::string *__fastcall `anonymous namespace'::CConsoleVariable<unsigned int>::GetHelp");
    assert_eq!(result, vec![
        "`anonymous namespace'",
        "CConsoleVariable<unsigned int>",
        "GetHelp"
    ]);
}

#[test]
fn test_signature_050() {
    let result = parse_signature("std::_Tree_const_iterator<std::_Tree_val<std::_Tmap_traits<void *,unsigned __int64,std::less<void *>,std::allocator<std::pair<void * const,unsigned __int64> >,0> > > *__fastcall std::_Tree_const_iterator<std::_Tree_val<std::_Tmap_traits<void *,unsigned __int64,std::less<void *>,std::allocator<std::pair<void * const,unsigned __int64>>,0>>>::operator++");
    assert_eq!(result, vec![
        "std",
        "_Tree_const_iterator<std::_Tree_val<std::_Tmap_traits<void *,unsigned __int64,std::less<void *>,std::allocator<std::pair<void * const,unsigned __int64>>,0>>>",
        "operator++"
    ]);
}

#[test]
fn test_signature_051() {
    let result = parse_signature("void __fastcall `anonymous namespace'::type_to_string");
    assert_eq!(result, vec![
        "`anonymous namespace'",
        "type_to_string"
    ]);
}

#[test]
fn test_signature_052() {
    let result = parse_signature("void __fastcall NGraphicsEngine::CLight::SetShadowCaster");
    assert_eq!(result, vec![
        "NGraphicsEngine",
        "CLight",
        "SetShadowCaster"
    ]);
}

#[test]
fn test_signature_053() {
    let result = parse_signature("void __fastcall std::_Fill_n<SLocationInfo * *,unsigned __int64,SLocationInfo *>");
    assert_eq!(result, vec![
        "std",
        "_Fill_n<SLocationInfo * *,unsigned __int64,SLocationInfo *>"
    ]);
}

#[test]
fn test_signature_054() {
    let result = parse_signature("void __fastcall CAnimatedModel::CAnimatedModelConstructionInfo::CAnimatedModelConstructionInfo");
    assert_eq!(result, vec![
        "CAnimatedModel",
        "CAnimatedModelConstructionInfo",
        "CAnimatedModelConstructionInfo"
    ]);
}

#[test]
fn test_signature_055() {
    let result = parse_signature("bool __fastcall CNetworkData::HasSender");
    assert_eq!(result, vec![
        "CNetworkData",
        "HasSender"
    ]);
}

#[test]
fn test_signature_056() {
    let result = parse_signature("void __fastcall boost::detail::sp_deleter_construct<CGameObject,CReverbController>");
    assert_eq!(result, vec![
        "boost",
        "detail",
        "sp_deleter_construct<CGameObject,CReverbController>"
    ]);
}

#[test]
fn test_signature_057() {
    let result = parse_signature("void __fastcall std::tr1::_Impl_no_alloc0<std::tr1::_Callable_obj<`anonymous namespace'::_lambda9_,0>,void>::_Impl_no_alloc0<std::tr1::_Callable_obj<`anonymous namespace'::_lambda9_,0>,void>");
    assert_eq!(result, vec![
        "std",
        "tr1",
        "_Impl_no_alloc0<std::tr1::_Callable_obj<`anonymous namespace'::_lambda9_,0>,void>",
        "_Impl_no_alloc0<std::tr1::_Callable_obj<`anonymous namespace'::_lambda9_,0>,void>"
    ]);
}

#[test]
fn test_signature_058() {
    let result = parse_signature("const CHashString *__fastcall CDestructionObjectKeyframed::GetClassHash");
    assert_eq!(result, vec![
        "CDestructionObjectKeyframed",
        "GetClassHash"
    ]);
}

#[test]
fn test_signature_059() {
    let result = parse_signature("const std::pair<unsigned int const ,std::pair<std::vector<unsigned int>,std::vector<std::string> > > *__fastcall std::_Tree_const_iterator<std::_Tree_val<std::_Tmap_traits<unsigned int,std::pair<std::vector<unsigned int>,std::vector<std::string>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,std::pair<std::vector<unsigned int>,std::vector<std::string>>>>,0>>>::operator*");
    assert_eq!(result, vec![
        "std",
        "_Tree_const_iterator<std::_Tree_val<std::_Tmap_traits<unsigned int,std::pair<std::vector<unsigned int>,std::vector<std::string>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,std::pair<std::vector<unsigned int>,std::vector<std::string>>>>,0>>>",
        "operator*"
    ]);
}

#[test]
fn test_signature_060() {
    let result = parse_signature("char __fastcall CDialogueManager::IsDialoguePlaying");
    assert_eq!(result, vec![
        "CDialogueManager",
        "IsDialoguePlaying"
    ]);
}

#[test]
fn test_signature_061() {
    let result = parse_signature("void __fastcall std::pair<std::_Tree_const_iterator<std::_Tree_val<std::_Tset_traits<unsigned int,std::less<unsigned int>,std::allocator<unsigned int>,0>>>,bool>::pair<std::_Tree_const_iterator<std::_Tree_val<std::_Tset_traits<unsigned int,std::less<unsigned int>,std::allocator<unsigned int>,0>>>,bool>");
    assert_eq!(result, vec![
        "std",
        "pair<std::_Tree_const_iterator<std::_Tree_val<std::_Tset_traits<unsigned int,std::less<unsigned int>,std::allocator<unsigned int>,0>>>,bool>",
        "pair<std::_Tree_const_iterator<std::_Tree_val<std::_Tset_traits<unsigned int,std::less<unsigned int>,std::allocator<unsigned int>,0>>>,bool>"
    ]);
}

#[test]
fn test_signature_062() {
    let result = parse_signature("void __fastcall std::vector<SGameObjectCreateInfo>::reserve");
    assert_eq!(result, vec![
        "std",
        "vector<SGameObjectCreateInfo>",
        "reserve"
    ]);
}

#[test]
fn test_signature_063() {
    let result = parse_signature("unsigned __int8 *__fastcall NGraphScript::CGraph::GetInternalData");
    assert_eq!(result, vec![
        "NGraphScript",
        "CGraph",
        "GetInternalData"
    ]);
}

#[test]
fn test_signature_064() {
    let result = parse_signature("void __fastcall boost::fusion::basic_iterator<boost::fusion::struct_iterator_tag,boost::fusion::random_access_traversal_tag,dynamo::ast::unary_op,0>::basic_iterator<boost::fusion::struct_iterator_tag,boost::fusion::random_access_traversal_tag,dynamo::ast::unary_op,0>");
    assert_eq!(result, vec![
        "boost",
        "fusion",
        "basic_iterator<boost::fusion::struct_iterator_tag,boost::fusion::random_access_traversal_tag,dynamo::ast::unary_op,0>",
        "basic_iterator<boost::fusion::struct_iterator_tag,boost::fusion::random_access_traversal_tag,dynamo::ast::unary_op,0>"
    ]);
}

#[test]
fn test_signature_065() {
    let result = parse_signature("boost::fusion::cons_iterator<boost::fusion::cons<boost::spirit::qi::reference<boost::spirit::qi::rule<char const *,dynamo::ast::identifier __cdecl(void),boost::proto::exprns_::expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<boost::spirit::tag::char_code<boost::spirit::tag::space,boost::spirit::char_encoding::ascii> >,0>,boost::spirit::unused_type,boost::spirit::unused_type> const >,boost::fusion::cons<boost::spirit::qi::expect<boost::fusion::cons<boost::spirit::qi::literal_char<boost::spirit::char_encoding::standard,1,0>,boost::fusion::cons<boost::spirit::qi::reference<boost::spirit::qi::rule<char const *,dynamo::ast::expression __cdecl(void),boost::proto::exprns_::expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<boost::spirit::tag::char_code<boost::spirit::tag::space,boost::spirit::char_encoding::ascii> >,0>,boost::spirit::unused_type,boost::spirit::unused_type> const >,boost::fusion::cons<boost::spirit::qi::literal_char<boost::spirit::char_encoding::standard,1,0>,boost::fusion::nil_> > > >,boost::fusion::nil_> > const > *__fastcall boost::fusion::extension::next_impl<boost::fusion::cons_iterator_tag>::apply<boost::fusion::cons_iterator<boost::fusion::cons<boost::spirit::qi::reference<boost::spirit::qi::rule<char const *,dynamo::ast::intrinsic_op (void),boost::proto::exprns_::expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<boost::spirit::tag::char_code<boost::spirit::tag::space,boost::spirit::char_encoding::ascii>>,0>,boost::spirit::unused_type,boost::spirit::unused_type> const>,boost::fusion::cons<boost::spirit::qi::reference<boost::spirit::qi::rule<char const *,dynamo::ast::identifier (void),boost::proto::exprns_::expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<boost::spirit::tag::char_code<boost::spirit::tag::space,boost::spirit::char_encoding::ascii>>,0>,boost::spirit::unused_type,boost::spirit::unused_type> const>,boost::fusion::cons<boost::spirit::qi::expect<boost::fusion::cons<boost::spirit::qi::literal_char<boost::spirit::char_encoding::standard,1,0>,boost::fusion::cons<boost::spirit::qi::reference<boost::spirit::qi::rule<char const *,dynamo::ast::expression (void),boost::proto::exprns_::expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<boost::spirit::tag::char_code<boost::spirit::tag::space,boost::spirit::char_encoding::ascii>>,0>,boost::spirit::unused_type,boost::spirit::unused_type> const>,boost::fusion::cons<boost::spirit::qi::literal_char<boost::spirit::char_encoding::standard,1,0>,boost::fusion::nil_>>>>,boost::fusion::nil_>>> const>>::call");
    assert_eq!(result, vec![
        "boost",
        "fusion",
        "extension",
        "next_impl<boost::fusion::cons_iterator_tag>",
        "apply<boost::fusion::cons_iterator<boost::fusion::cons<boost::spirit::qi::reference<boost::spirit::qi::rule<char const *,dynamo::ast::intrinsic_op (void),boost::proto::exprns_::expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<boost::spirit::tag::char_code<boost::spirit::tag::space,boost::spirit::char_encoding::ascii>>,0>,boost::spirit::unused_type,boost::spirit::unused_type> const>,boost::fusion::cons<boost::spirit::qi::reference<boost::spirit::qi::rule<char const *,dynamo::ast::identifier (void),boost::proto::exprns_::expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<boost::spirit::tag::char_code<boost::spirit::tag::space,boost::spirit::char_encoding::ascii>>,0>,boost::spirit::unused_type,boost::spirit::unused_type> const>,boost::fusion::cons<boost::spirit::qi::expect<boost::fusion::cons<boost::spirit::qi::literal_char<boost::spirit::char_encoding::standard,1,0>,boost::fusion::cons<boost::spirit::qi::reference<boost::spirit::qi::rule<char const *,dynamo::ast::expression (void),boost::proto::exprns_::expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<boost::spirit::tag::char_code<boost::spirit::tag::space,boost::spirit::char_encoding::ascii>>,0>,boost::spirit::unused_type,boost::spirit::unused_type> const>,boost::fusion::cons<boost::spirit::qi::literal_char<boost::spirit::char_encoding::standard,1,0>,boost::fusion::nil_>>>>,boost::fusion::nil_>>> const>>",
        "call"
    ]);
}

#[test]
fn test_signature_066() {
    let result = parse_signature("const char *const *__fastcall boost::fusion::at<boost::mpl::int_<2>,boost::fusion::vector<char const * &,char const * const &,char const * const &,boost::spirit::info const &,boost::fusion::void_,boost::fusion::void_,boost::fusion::void_,boost::fusion::void_,boost::fusion::void_,boost::fusion::void_>>");
    assert_eq!(result, vec![
        "boost",
        "fusion",
        "at<boost::mpl::int_<2>,boost::fusion::vector<char const * &,char const * const &,char const * const &,boost::spirit::info const &,boost::fusion::void_,boost::fusion::void_,boost::fusion::void_,boost::fusion::void_,boost::fusion::void_,boost::fusion::void_>>"
    ]);
}

#[test]
fn test_signature_067() {
    let result = parse_signature("boost::spirit::qi::sequence<boost::fusion::cons<boost::spirit::qi::reference<boost::spirit::qi::symbols<char,enum dynamo::ast::op_token,boost::spirit::qi::tst<char,enum dynamo::ast::op_token>,boost::spirit::qi::tst_pass_through> >,boost::fusion::cons<boost::spirit::qi::reference<boost::spirit::qi::rule<char const *,boost::variant<dynamo::ast::nil,float,dynamo::ast::identifier,boost::recursive_wrapper<dynamo::ast::unary_op>,boost::recursive_wrapper<dynamo::ast::binary_op>,boost::recursive_wrapper<dynamo::ast::intrinsic_op>,boost::recursive_wrapper<dynamo::ast::expression>> __cdecl(void),boost::proto::exprns_::expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<boost::spirit::tag::char_code<boost::spirit::tag::space,boost::spirit::char_encoding::ascii> >,0>,boost::spirit::unused_type,boost::spirit::unused_type> const >,boost::fusion::nil_> > > *__fastcall boost::proto::transform<boost::proto::switch_<boost::spirit::meta_compiler<boost::spirit::qi::domain>::cases,boost::proto::tag_of<boost::proto::_> (void)>,void>::operator()<boost::proto::exprns_::expr<boost::proto::tagns_::tag::shift_right,boost::proto::argsns_::list2<boost::spirit::qi::symbols<char,enum dynamo::ast::op_token,boost::spirit::qi::tst<char,enum dynamo::ast::op_token>,boost::spirit::qi::tst_pass_through> &,boost::spirit::qi::rule<char const *,boost::variant<dynamo::ast::nil,float,dynamo::ast::identifier,boost::recursive_wrapper<dynamo::ast::unary_op>,boost::recursive_wrapper<dynamo::ast::binary_op>,boost::recursive_wrapper<dynamo::ast::intrinsic_op>,boost::recursive_wrapper<dynamo::ast::expression>> (void),boost::proto::exprns_::expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<boost::spirit::tag::char_code<boost::spirit::tag::space,boost::spirit::char_encoding::ascii>>,0>,boost::spirit::unused_type,boost::spirit::unused_type> &>,2> const &,boost::mpl::void_,boost::spirit::unused_type &>");
    assert_eq!(result, vec![
        "boost",
        "proto",
        "transform<boost::proto::switch_<boost::spirit::meta_compiler<boost::spirit::qi::domain>::cases,boost::proto::tag_of<boost::proto::_> (void)>,void>",
        "operator()<boost::proto::exprns_::expr<boost::proto::tagns_::tag::shift_right,boost::proto::argsns_::list2<boost::spirit::qi::symbols<char,enum dynamo::ast::op_token,boost::spirit::qi::tst<char,enum dynamo::ast::op_token>,boost::spirit::qi::tst_pass_through> &,boost::spirit::qi::rule<char const *,boost::variant<dynamo::ast::nil,float,dynamo::ast::identifier,boost::recursive_wrapper<dynamo::ast::unary_op>,boost::recursive_wrapper<dynamo::ast::binary_op>,boost::recursive_wrapper<dynamo::ast::intrinsic_op>,boost::recursive_wrapper<dynamo::ast::expression>> (void),boost::proto::exprns_::expr<boost::proto::tagns_::tag::terminal,boost::proto::argsns_::term<boost::spirit::tag::char_code<boost::spirit::tag::space,boost::spirit::char_encoding::ascii>>,0>,boost::spirit::unused_type,boost::spirit::unused_type> &>,2> const &,boost::mpl::void_,boost::spirit::unused_type &>"
    ]);
}

#[test]
fn test_signature_068() {
    let result = parse_signature("void __fastcall boost::spirit::traits::detail::assign_to<boost::variant<dynamo::ast::assignment,dynamo::ast::expression>,boost::variant<dynamo::ast::nil,dynamo::ast::assignment,dynamo::ast::expression>>");
    assert_eq!(result, vec![
        "boost",
        "spirit",
        "traits",
        "detail",
        "assign_to<boost::variant<dynamo::ast::assignment,dynamo::ast::expression>,boost::variant<dynamo::ast::nil,dynamo::ast::assignment,dynamo::ast::expression>>"
    ]);
}

#[test]
fn test_signature_069() {
    let result = parse_signature("CContextAction **__fastcall std::_Allocate<CContextAction *>");
    assert_eq!(result, vec![
        "std",
        "_Allocate<CContextAction *>"
    ]);
}

#[test]
fn test_signature_070() {
    let result = parse_signature("__int64 __fastcall AiVehicleCloseDoor");
    assert_eq!(result, vec![
        "AiVehicleCloseDoor"
    ]);
}

#[test]
fn test_signature_071() {
    let result = parse_signature("const ava::idstring<CRtti::SRttiTAG,0> *__fastcall CContextAction::TYPE_ID");
    assert_eq!(result, vec![
        "CContextAction",
        "TYPE_ID"
    ]);
}

#[test]
fn test_signature_072() {
    let result = parse_signature("std::_Unique_ptr_base<unsigned char,CHavokAiWorld::CNavMeshResourceDeleter<1>,1> *__fastcall std::_Unique_ptr_base<unsigned char,CHavokAiWorld::CNavMeshResourceDeleter<1>,1>::get_deleter");
    assert_eq!(result, vec![
        "std",
        "_Unique_ptr_base<unsigned char,CHavokAiWorld::CNavMeshResourceDeleter<1>,1>",
        "get_deleter"
    ]);
}

#[test]
fn test_signature_073() {
    let result = parse_signature("void __fastcall hkaiSearchStateNode::setOpen");
    assert_eq!(result, vec![
        "hkaiSearchStateNode",
        "setOpen"
    ]);
}

#[test]
fn test_signature_074() {
    let result = parse_signature("void __fastcall std::_Deque_const_iterator<unsigned int>::_Deque_const_iterator<unsigned int>");
    assert_eq!(result, vec![
        "std",
        "_Deque_const_iterator<unsigned int>",
        "_Deque_const_iterator<unsigned int>"
    ]);
}

#[test]
fn test_signature_075() {
    let result = parse_signature("void __fastcall CAINavmeshPathfinder::AddPathfindRequest");
    assert_eq!(result, vec![
        "CAINavmeshPathfinder",
        "AddPathfindRequest"
    ]);
}

#[test]
fn test_signature_076() {
    let result = parse_signature("std::pair<CHashString,CHashString> *__fastcall CStatisticContext::allocator_t<std::pair<CHashString,CHashString>>::allocate");
    assert_eq!(result, vec![
        "CStatisticContext",
        "allocator_t<std::pair<CHashString,CHashString>>",
        "allocate"
    ]);
}

#[test]
fn test_signature_077() {
    let result = parse_signature("boost::container::container_detail::flat_tree_value_compare<std::less<unsigned int>,boost::container::container_detail::pair<unsigned int,bool>,boost::container::container_detail::select1st<boost::container::container_detail::pair<unsigned int,bool> > > *__fastcall boost::container::container_detail::flat_tree_value_compare<std::less<unsigned int>,boost::container::container_detail::pair<unsigned int,bool>,boost::container::container_detail::select1st<boost::container::container_detail::pair<unsigned int,bool>>>::get_comp");
    assert_eq!(result, vec![
        "boost",
        "container",
        "container_detail",
        "flat_tree_value_compare<std::less<unsigned int>,boost::container::container_detail::pair<unsigned int,bool>,boost::container::container_detail::select1st<boost::container::container_detail::pair<unsigned int,bool>>>",
        "get_comp"
    ]);
}

#[test]
fn test_signature_078() {
    let result = parse_signature("void *__fastcall boost::detail::sp_counted_impl_p<CCameraAnimationModifier::CDummyGameObject>::get_deleter");
    assert_eq!(result, vec![
        "boost",
        "detail",
        "sp_counted_impl_p<CCameraAnimationModifier::CDummyGameObject>",
        "get_deleter"
    ]);
}

#[test]
fn test_signature_079() {
    let result = parse_signature("char __fastcall CGameCameraManager::GetValue<int>");
    assert_eq!(result, vec![
        "CGameCameraManager",
        "GetValue<int>"
    ]);
}

#[test]
fn test_signature_080() {
    let result = parse_signature("void __fastcall CMoveInputPitchModifier::EnterUnalignedTransition");
    assert_eq!(result, vec![
        "CMoveInputPitchModifier",
        "EnterUnalignedTransition"
    ]);
}

#[test]
fn test_signature_081() {
    let result = parse_signature("void __fastcall std::list<std::pair<CStatisticContext const,float>>::list<std::pair<CStatisticContext const,float>>");
    assert_eq!(result, vec![
        "std",
        "list<std::pair<CStatisticContext const,float>>",
        "list<std::pair<CStatisticContext const,float>>"
    ]);
}

#[test]
fn test_signature_082() {
    let result = parse_signature("void __fastcall boost::container::vector<boost::container::container_detail::pair<CHashString,CStatisticDelegateHolder<TDelegate<void (CStatistic<__int64> const *,__int64,__int64,CStatisticContext const &)>>>,std::allocator<boost::container::container_detail::pair<CHashString,CStatisticDelegateHolder<TDelegate<void (CStatistic<__int64> const *,__int64,__int64,CStatisticContext const &)>>>>>::priv_forward_range_insert_new_allocation<boost::container::container_detail::insert_move_proxy<std::allocator<boost::container::container_detail::pair<CHashString,CStatisticDelegateHolder<TDelegate<void (CStatistic<__int64> const *,__int64,__int64,CStatisticContext const &)>>>>,boost::container::container_detail::pair<CHashString,CStatisticDelegateHolder<TDelegate<void (CStatistic<__int64> const *,__int64,__int64,CStatisticContext const &)>>> *>>");
    assert_eq!(result, vec![
        "boost",
        "container",
        "vector<boost::container::container_detail::pair<CHashString,CStatisticDelegateHolder<TDelegate<void (CStatistic<__int64> const *,__int64,__int64,CStatisticContext const &)>>>,std::allocator<boost::container::container_detail::pair<CHashString,CStatisticDelegateHolder<TDelegate<void (CStatistic<__int64> const *,__int64,__int64,CStatisticContext const &)>>>>>",
        "priv_forward_range_insert_new_allocation<boost::container::container_detail::insert_move_proxy<std::allocator<boost::container::container_detail::pair<CHashString,CStatisticDelegateHolder<TDelegate<void (CStatistic<__int64> const *,__int64,__int64,CStatisticContext const &)>>>>,boost::container::container_detail::pair<CHashString,CStatisticDelegateHolder<TDelegate<void (CStatistic<__int64> const *,__int64,__int64,CStatisticContext const &)>>> *>>"
    ]);
}

#[test]
fn test_signature_083() {
    let result = parse_signature("NAr::CVersionedBinaryInputArchive<NAr::CBlobStream> *__fastcall NAr::CVersionedBinaryInputArchive<NAr::CBlobStream>::load<__int64>");
    assert_eq!(result, vec![
        "NAr",
        "CVersionedBinaryInputArchive<NAr::CBlobStream>",
        "load<__int64>"
    ]);
}

#[test]
fn test_signature_084() {
    let result = parse_signature("NAr::detail::CVersionedBinaryInputArchiveImpl<NSaveLoad::CUserDataStream> *__fastcall NAr::detail::CVersionedBinaryInputArchiveImpl<NSaveLoad::CUserDataStream>::operator()<std::tr1::unordered_map<CStatisticContext,float,std::hash<CStatisticContext>,std::equal_to<CStatisticContext>,std::allocator<std::pair<CStatisticContext const,float>>>>");
    assert_eq!(result, vec![
        "NAr",
        "detail",
        "CVersionedBinaryInputArchiveImpl<NSaveLoad::CUserDataStream>",
        "operator()<std::tr1::unordered_map<CStatisticContext,float,std::hash<CStatisticContext>,std::equal_to<CStatisticContext>,std::allocator<std::pair<CStatisticContext const,float>>>>"
    ]);
}

#[test]
fn test_signature_085() {
    let result = parse_signature("bool __fastcall CLinkTargetEventCondition::UnRegister");
    assert_eq!(result, vec![
        "CLinkTargetEventCondition",
        "UnRegister"
    ]);
}

#[test]
fn test_signature_086() {
    let result = parse_signature("CBoneTransformEventSender *__fastcall CBoneTransformEventSender::`scalar deleting destructor'");
    assert_eq!(result, vec![
        "CBoneTransformEventSender",
        "`scalar deleting destructor'"
    ]);
}

#[test]
fn test_signature_087() {
    let result = parse_signature("boost::shared_ptr<CWeaponPlacementRule> *__fastcall boost::static_pointer_cast<CWeaponPlacementRule,CGameObject>");
    assert_eq!(result, vec![
        "boost",
        "static_pointer_cast<CWeaponPlacementRule,CGameObject>"
    ]);
}

#[test]
fn test_signature_088() {
    let result = parse_signature("__int64 __fastcall CVehicleBelowCondition::Register");
    assert_eq!(result, vec![
        "CVehicleBelowCondition",
        "Register"
    ]);
}

#[test]
fn test_signature_089() {
    let result = parse_signature("CVector3f *__fastcall CPlantedExplosiveWeapon::GetCharacterWarpTargetPosition");
    assert_eq!(result, vec![
        "CPlantedExplosiveWeapon",
        "GetCharacterWarpTargetPosition"
    ]);
}

#[test]
fn test_signature_090() {
    let result = parse_signature("void __fastcall CDiveState::CDiveState");
    assert_eq!(result, vec![
        "CDiveState",
        "CDiveState"
    ]);
}

#[test]
fn test_signature_091() {
    let result = parse_signature("void __fastcall std::allocator<boost::shared_ptr<CDamageControllerArmor>>::allocator<boost::shared_ptr<CDamageControllerArmor>>");
    assert_eq!(result, vec![
        "std",
        "allocator<boost::shared_ptr<CDamageControllerArmor>>",
        "allocator<boost::shared_ptr<CDamageControllerArmor>>"
    ]);
}

#[test]
fn test_signature_092() {
    let result = parse_signature("void __fastcall CCollectionManager::SCollectibleRecord::~SCollectibleRecord");
    assert_eq!(result, vec![
        "CCollectionManager",
        "SCollectibleRecord",
        "~SCollectibleRecord"
    ]);
}

#[test]
fn test_signature_093() {
    let result = parse_signature("void __fastcall std::_Deque_iterator<std::pair<STaskInfo const *,__int64>>::_Deque_iterator<std::pair<STaskInfo const *,__int64>>");
    assert_eq!(result, vec![
        "std",
        "_Deque_iterator<std::pair<STaskInfo const *,__int64>>",
        "_Deque_iterator<std::pair<STaskInfo const *,__int64>>"
    ]);
}

#[test]
fn test_signature_094() {
    let result = parse_signature("void __fastcall std::_Dest_val<std::allocator<std::tr1::function<void (CCallContext &)>>,std::tr1::function<void (CCallContext &)>>");
    assert_eq!(result, vec![
        "std",
        "_Dest_val<std::allocator<std::tr1::function<void (CCallContext &)>>,std::tr1::function<void (CCallContext &)>>"
    ]);
}

#[test]
fn test_signature_095() {
    let result = parse_signature("SAsyncCallProxy<CCallContext> *__fastcall NOnline::detail::ExecuteAsyncImpl<A0x78becc47::`anonymous namespace'::_lambda22_,CCallContext>");
    assert_eq!(result, vec![
        "NOnline",
        "detail",
        "ExecuteAsyncImpl<A0x78becc47::`anonymous namespace'::_lambda22_,CCallContext>"
    ]);
}

#[test]
fn test_signature_096() {
    let result = parse_signature("bool __fastcall CGrapplingHangAttachmentProxy::GetVisualTransformWS");
    assert_eq!(result, vec![
        "CGrapplingHangAttachmentProxy",
        "GetVisualTransformWS"
    ]);
}

#[test]
fn test_signature_097() {
    let result = parse_signature("void __fastcall CMissile::SetMissileTarget");
    assert_eq!(result, vec![
        "CMissile",
        "SetMissileTarget"
    ]);
}

#[test]
fn test_signature_098() {
    let result = parse_signature("bool __fastcall CGrapplingHookPropData::CheckBreakThresholdReached");
    assert_eq!(result, vec![
        "CGrapplingHookPropData",
        "CheckBreakThresholdReached"
    ]);
}

#[test]
fn test_signature_099() {
    let result = parse_signature("void __fastcall CCharacter::RemoveTether");
    assert_eq!(result, vec![
        "CCharacter",
        "RemoveTether"
    ]);
}

#[test]
fn test_signature_100() {
    let result = parse_signature("signed __int64 __fastcall TArray<CWeaponData *>::GetCapacity");
    assert_eq!(result, vec![
        "TArray<CWeaponData *>",
        "GetCapacity"
    ]);
}

#[test]
fn test_signature_101() {
    let result = parse_signature("boost::shared_ptr<CShapeTrigger> *__fastcall boost::static_pointer_cast<CShapeTrigger,CGameObject>");
    assert_eq!(result, vec![
        "boost",
        "static_pointer_cast<CShapeTrigger,CGameObject>"
    ]);
}

#[test]
fn test_signature_102() {
    let result = parse_signature("const CHashString *__fastcall CBeaconWeapon::GetClassHash");
    assert_eq!(result, vec![
        "CBeaconWeapon",
        "GetClassHash"
    ]);
}

#[test]
fn test_signature_103() {
    let result = parse_signature("void __fastcall ALIGNED16_NEW<CProjectEffectToTerrain>");
    assert_eq!(result, vec![
        "ALIGNED16_NEW<CProjectEffectToTerrain>"
    ]);
}

#[test]
fn test_signature_104() {
    let result = parse_signature("void __fastcall boost::detail::sp_counted_impl_pd<CLocationChecker *,CGameObjectCreator<CLocationChecker>::SGameObjectDestroyer>::dispose");
    assert_eq!(result, vec![
        "boost",
        "detail",
        "sp_counted_impl_pd<CLocationChecker *,CGameObjectCreator<CLocationChecker>::SGameObjectDestroyer>",
        "dispose"
    ]);
}

#[test]
fn test_signature_105() {
    let result = parse_signature("void __fastcall boost::detail::sp_counted_impl_pd<CEncounterObject *,CGameObjectCreator<CEncounterObject>::SGameObjectDestroyer>::~sp_counted_impl_pd<CEncounterObject *,CGameObjectCreator<CEncounterObject>::SGameObjectDestroyer>");
    assert_eq!(result, vec![
        "boost",
        "detail",
        "sp_counted_impl_pd<CEncounterObject *,CGameObjectCreator<CEncounterObject>::SGameObjectDestroyer>",
        "~sp_counted_impl_pd<CEncounterObject *,CGameObjectCreator<CEncounterObject>::SGameObjectDestroyer>"
    ]);
}

#[test]
fn test_signature_106() {
    let result = parse_signature("CGameObjectCreator<CCar> *__fastcall CGameObjectCreator<CCar>::`scalar deleting destructor'");
    assert_eq!(result, vec![
        "CGameObjectCreator<CCar>",
        "`scalar deleting destructor'"
    ]);
}

#[test]
fn test_signature_107() {
    let result = parse_signature("void __fastcall std::tr1::function<std::unique_ptr<CPfxRigidBody> (hknpBodyId)>::~function<std::unique_ptr<CPfxRigidBody> (hknpBodyId)>");
    assert_eq!(result, vec![
        "std",
        "tr1",
        "function<std::unique_ptr<CPfxRigidBody> (hknpBodyId)>",
        "~function<std::unique_ptr<CPfxRigidBody> (hknpBodyId)>"
    ]);
}

#[test]
fn test_signature_108() {
    let result = parse_signature("std::pair<std::string,Base::IAppSystem *> *__fastcall std::_Move<std::pair<std::string,Base::IAppSystem *> *,std::pair<std::string,Base::IAppSystem *> *>");
    assert_eq!(result, vec![
        "std",
        "_Move<std::pair<std::string,Base::IAppSystem *> *,std::pair<std::string,Base::IAppSystem *> *>"
    ]);
}

#[test]
fn test_signature_109() {
    let result = parse_signature("void __fastcall CVehicleRule::SetStartSpeed");
    assert_eq!(result, vec![
        "CVehicleRule",
        "SetStartSpeed"
    ]);
}

#[test]
fn test_signature_110() {
    let result = parse_signature("CInteractionGraphIsAttachedCondition *__fastcall CInteractionGraphIsAttachedCondition::`vector deleting destructor'");
    assert_eq!(result, vec![
        "CInteractionGraphIsAttachedCondition",
        "`vector deleting destructor'"
    ]);
}

#[test]
fn test_signature_111() {
    let result = parse_signature("const NRevolution::SSettlementRecord *__fastcall std::_Vector_const_iterator<std::_Vector_val<NRevolution::SSettlementRecord>>::operator*");
    assert_eq!(result, vec![
        "std",
        "_Vector_const_iterator<std::_Vector_val<NRevolution::SSettlementRecord>>",
        "operator*"
    ]);
}

#[test]
fn test_signature_112() {
    let result = parse_signature("float __fastcall Absf_32");
    assert_eq!(result, vec![
        "Absf_32"
    ]);
}

#[test]
fn test_signature_113() {
    let result = parse_signature("bool __fastcall std::_Vector_const_iterator<std::_Vector_val<NMissionSystem::MissionWeaponCacheEntry>>::operator!=");
    assert_eq!(result, vec![
        "std",
        "_Vector_const_iterator<std::_Vector_val<NMissionSystem::MissionWeaponCacheEntry>>",
        "operator!="
    ]);
}

#[test]
fn test_signature_114() {
    let result = parse_signature("bool __fastcall NMissionSystem::CMissionManager::IsSystemForced");
    assert_eq!(result, vec![
        "NMissionSystem",
        "CMissionManager",
        "IsSystemForced"
    ]);
}

#[test]
fn test_signature_115() {
    let result = parse_signature("void __fastcall std::tr1::_Callable_base<`anonymous namespace'::_lambda24_,0>::_Callable_base<`anonymous namespace'::_lambda24_,0>");
    assert_eq!(result, vec![
        "std",
        "tr1",
        "_Callable_base<`anonymous namespace'::_lambda24_,0>",
        "_Callable_base<`anonymous namespace'::_lambda24_,0>"
    ]);
}

#[test]
fn test_signature_116() {
    let result = parse_signature("__int64 __fastcall CNetVar_template<bool>::getDataSize");
    assert_eq!(result, vec![
        "CNetVar_template<bool>",
        "getDataSize"
    ]);
}

#[test]
fn test_signature_117() {
    let result = parse_signature("void __fastcall CNetworkData::Read<enum PlayerNetworkEvent>");
    assert_eq!(result, vec![
        "CNetworkData",
        "Read<enum PlayerNetworkEvent>"
    ]);
}

#[test]
fn test_signature_118() {
    let result = parse_signature("NAr::CBinaryOutputArchive<NSaveLoad::CUserDataStream> *__fastcall NAr::CBinaryOutputArchive<NSaveLoad::CUserDataStream>::store<double>");
    assert_eq!(result, vec![
        "NAr",
        "CBinaryOutputArchive<NSaveLoad::CUserDataStream>",
        "store<double>"
    ]);
}

#[test]
fn test_signature_119() {
    let result = parse_signature("char __fastcall std::tr1::_Impl_no_alloc0<std::tr1::_Callable_obj<A0x80bf83e1::`anonymous namespace'::_lambda16_,0>,bool>::_Do_call");
    assert_eq!(result, vec![
        "std",
        "tr1",
        "_Impl_no_alloc0<std::tr1::_Callable_obj<A0x80bf83e1::`anonymous namespace'::_lambda16_,0>,bool>",
        "_Do_call"
    ]);
}

#[test]
fn test_signature_120() {
    let result = parse_signature("std::vector<boost::shared_ptr<CSessionInfo>> *__fastcall std::move<std::vector<boost::shared_ptr<CSessionInfo>> &>");
    assert_eq!(result, vec![
        "std",
        "move<std::vector<boost::shared_ptr<CSessionInfo>> &>"
    ]);
}

#[test]
fn test_signature_121() {
    let result = parse_signature("boost::container::container_detail::pair<CCallContext const *,std::pair<CPlaylist const *,std::vector<boost::shared_ptr<CSessionInfo>> *> > *__fastcall boost::container::container_detail::iterator_to_pointer<boost::container::container_detail::pair<CCallContext const *,std::pair<CPlaylist const *,std::vector<boost::shared_ptr<CSessionInfo>> *>>>");
    assert_eq!(result, vec![
        "boost",
        "container",
        "container_detail",
        "iterator_to_pointer<boost::container::container_detail::pair<CCallContext const *,std::pair<CPlaylist const *,std::vector<boost::shared_ptr<CSessionInfo>> *>>>"
    ]);
}

#[test]
fn test_signature_122() {
    let result = parse_signature("void __fastcall std::unique_ptr<CMatchmakingProvider>::reset");
    assert_eq!(result, vec![
        "std",
        "unique_ptr<CMatchmakingProvider>",
        "reset"
    ]);
}

#[test]
fn test_signature_123() {
    let result = parse_signature("std::_Deque_iterator<char> *__fastcall std::deque<char>::begin");
    assert_eq!(result, vec![
        "std",
        "deque<char>",
        "begin"
    ]);
}

#[test]
fn test_signature_124() {
    let result = parse_signature("void __fastcall std::vector<SDlcSettings>::~vector<SDlcSettings>");
    assert_eq!(result, vec![
        "std",
        "vector<SDlcSettings>",
        "~vector<SDlcSettings>"
    ]);
}

#[test]
fn test_signature_125() {
    let result = parse_signature("bool __fastcall CRigidObject::IsChaosObject");
    assert_eq!(result, vec![
        "CRigidObject",
        "IsChaosObject"
    ]);
}

#[test]
fn test_signature_126() {
    let result = parse_signature("boost::container::container_detail::pair<CHashString,SBucketDefinition *> *__fastcall boost::move<boost::container::container_detail::pair<CHashString,SBucketDefinition *> &>");
    assert_eq!(result, vec![
        "boost",
        "move<boost::container::container_detail::pair<CHashString,SBucketDefinition *> &>"
    ]);
}

#[test]
fn test_signature_127() {
    let result = parse_signature("void __fastcall std::allocator<std::tr1::_Impl_no_alloc0<std::tr1::_Callable_obj<`anonymous namespace'::_lambda90_,0>,void>>::allocator<std::tr1::_Impl_no_alloc0<std::tr1::_Callable_obj<`anonymous namespace'::_lambda90_,0>,void>>");
    assert_eq!(result, vec![
        "std",
        "allocator<std::tr1::_Impl_no_alloc0<std::tr1::_Callable_obj<`anonymous namespace'::_lambda90_,0>,void>>",
        "allocator<std::tr1::_Impl_no_alloc0<std::tr1::_Callable_obj<`anonymous namespace'::_lambda90_,0>,void>>"
    ]);
}

#[test]
fn test_signature_128() {
    let result = parse_signature("std::_Vector_const_iterator<std::_Vector_val<CStatisticMarshaler::SRegisterInfo<TDelegate<void __cdecl(CStatistic<float> const *,float,float,CStatisticContext const &)> >> > *__fastcall std::vector<CStatisticMarshaler::SRegisterInfo<TDelegate<void (CStatistic<float> const *,float,float,CStatisticContext const &)>>>::end");
    assert_eq!(result, vec![
        "std",
        "vector<CStatisticMarshaler::SRegisterInfo<TDelegate<void (CStatistic<float> const *,float,float,CStatisticContext const &)>>>",
        "end"
    ]);
}

#[test]
fn test_signature_129() {
    let result = parse_signature("std::_Vector_iterator<std::_Vector_val<unsigned __int64> > *__fastcall std::vector<unsigned __int64>::erase");
    assert_eq!(result, vec![
        "std",
        "vector<unsigned __int64>",
        "erase"
    ]);
}

#[test]
fn test_signature_130() {
    let result = parse_signature("void __fastcall std::_Dest_val<std::allocator<std::tr1::_Impl_no_alloc0<std::tr1::_Callable_obj<`anonymous namespace'::_lambda86_,0>,void>>,std::tr1::_Impl_no_alloc0<std::tr1::_Callable_obj<`anonymous namespace'::_lambda86_,0>,void>>");
    assert_eq!(result, vec![
        "std",
        "_Dest_val<std::allocator<std::tr1::_Impl_no_alloc0<std::tr1::_Callable_obj<`anonymous namespace'::_lambda86_,0>,void>>,std::tr1::_Impl_no_alloc0<std::tr1::_Callable_obj<`anonymous namespace'::_lambda86_,0>,void>>"
    ]);
}

#[test]
fn test_signature_131() {
    let result = parse_signature("void __fastcall CAnimatedTexture::CAnimatedTexture");
    assert_eq!(result, vec![
        "CAnimatedTexture",
        "CAnimatedTexture"
    ]);
}

#[test]
fn test_signature_132() {
    let result = parse_signature("_BOOL8 __fastcall CMagnetWeaponComponent::IsWeaponFiring");
    assert_eq!(result, vec![
        "CMagnetWeaponComponent",
        "IsWeaponFiring"
    ]);
}

#[test]
fn test_signature_133() {
    let result = parse_signature("unsigned __int64 *__fastcall boost::container::container_detail::vector_alloc_holder<std::allocator<boost::container::container_detail::pair<CHashString,SProfileItemInfo *>>,boost::container::container_detail::integral_constant<unsigned int,1>>::capacity");
    assert_eq!(result, vec![
        "boost",
        "container",
        "container_detail",
        "vector_alloc_holder<std::allocator<boost::container::container_detail::pair<CHashString,SProfileItemInfo *>>,boost::container::container_detail::integral_constant<unsigned int,1>>",
        "capacity"
    ]);
}

#[test]
fn test_signature_134() {
    let result = parse_signature("void __fastcall std::_Pair_base<std::_Tree_const_iterator<std::_Tree_val<std::_Tset_traits<CMissile *,std::less<CMissile *>,std::allocator<CMissile *>,0>>>,bool>::_Pair_base<std::_Tree_const_iterator<std::_Tree_val<std::_Tset_traits<CMissile *,std::less<CMissile *>,std::allocator<CMissile *>,0>>>,bool>");
    assert_eq!(result, vec![
        "std",
        "_Pair_base<std::_Tree_const_iterator<std::_Tree_val<std::_Tset_traits<CMissile *,std::less<CMissile *>,std::allocator<CMissile *>,0>>>,bool>",
        "_Pair_base<std::_Tree_const_iterator<std::_Tree_val<std::_Tset_traits<CMissile *,std::less<CMissile *>,std::allocator<CMissile *>,0>>>,bool>"
    ]);
}

#[test]
fn test_signature_135() {
    let result = parse_signature("void __fastcall CBitField<4>::Set");
    assert_eq!(result, vec![
        "CBitField<4>",
        "Set"
    ]);
}

#[test]
fn test_signature_136() {
    let result = parse_signature("boost::detail::sp_counted_impl_p<CBreakableTreePatch> *__fastcall boost::detail::sp_counted_impl_p<CBreakableTreePatch>::`scalar deleting destructor'");
    assert_eq!(result, vec![
        "boost",
        "detail",
        "sp_counted_impl_p<CBreakableTreePatch>",
        "`scalar deleting destructor'"
    ]);
}

#[test]
fn test_signature_137() {
    let result = parse_signature("std::tr1::_Callable_obj<<lambda5>,0> *__fastcall std::tr1::_Impl_no_alloc1<std::tr1::_Callable_obj<`anonymous namespace'::_lambda5_,0>,void,CVehicle *>::_Get");
    assert_eq!(result, vec![
        "std",
        "tr1",
        "_Impl_no_alloc1<std::tr1::_Callable_obj<`anonymous namespace'::_lambda5_,0>,void,CVehicle *>",
        "_Get"
    ]);
}

#[test]
fn test_signature_138() {
    let result = parse_signature("std::tr1::_Callable_obj<std::tr1::_Bind<void,void,std::tr1::_Bind1<std::tr1::_Callable_pmf<void (__cdecl NSaveLoad::CManagedObject<NSaveLoad::CLocalDataRef<CPlayerActionObserver::CSystemicButtonHintData> >::*const)(void),NSaveLoad::CManagedObject<NSaveLoad::CLocalDataRef<CPlayerActionObserver::CSystemicButtonHintData> >,0>,NSaveLoad::CManagedObject<NSaveLoad::CLocalDataRef<CPlayerActionObserver::CSystemicButtonHintData> > *> >,0> *__fastcall std::tr1::_Impl_no_alloc0<std::tr1::_Callable_obj<std::tr1::_Bind<void,void,std::tr1::_Bind1<std::tr1::_Callable_pmf<void (NSaveLoad::CManagedObject<NSaveLoad::CLocalDataRef<CPlayerActionObserver::CSystemicButtonHintData>>::*const)(void),NSaveLoad::CManagedObject<NSaveLoad::CLocalDataRef<CPlayerActionObserver::CSystemicButtonHintData>>,0>,NSaveLoad::CManagedObject<NSaveLoad::CLocalDataRef<CPlayerActionObserver::CSystemicButtonHintData>> *>>,0>,void>::_Get");
    assert_eq!(result, vec![
        "std",
        "tr1",
        "_Impl_no_alloc0<std::tr1::_Callable_obj<std::tr1::_Bind<void,void,std::tr1::_Bind1<std::tr1::_Callable_pmf<void (NSaveLoad::CManagedObject<NSaveLoad::CLocalDataRef<CPlayerActionObserver::CSystemicButtonHintData>>::*const)(void),NSaveLoad::CManagedObject<NSaveLoad::CLocalDataRef<CPlayerActionObserver::CSystemicButtonHintData>>,0>,NSaveLoad::CManagedObject<NSaveLoad::CLocalDataRef<CPlayerActionObserver::CSystemicButtonHintData>> *>>,0>,void>",
        "_Get"
    ]);
}

#[test]
fn test_signature_139() {
    let result = parse_signature("void __fastcall std::allocator<STaskInfo::SPrerequisite>::construct<STaskInfo::SPrerequisite &>");
    assert_eq!(result, vec![
        "std",
        "allocator<STaskInfo::SPrerequisite>",
        "construct<STaskInfo::SPrerequisite &>"
    ]);
}

#[test]
fn test_signature_140() {
    let result = parse_signature("CCallContext_OSuite *__fastcall CProfileProvider_OSuite::ConsumeItem");
    assert_eq!(result, vec![
        "CProfileProvider_OSuite",
        "ConsumeItem"
    ]);
}

#[test]
fn test_signature_141() {
    let result = parse_signature("void __fastcall CRegion::~CRegion");
    assert_eq!(result, vec![
        "CRegion",
        "~CRegion"
    ]);
}

#[test]
fn test_signature_142() {
    let result = parse_signature("void __fastcall boost::container::container_detail::vec_iterator<std::pair<CHashString,boost::container::static_vector<std::pair<PlayerId_Steam,int>,5>> *,1>::vec_iterator<std::pair<CHashString,boost::container::static_vector<std::pair<PlayerId_Steam,int>,5>> *,1>");
    assert_eq!(result, vec![
        "boost",
        "container",
        "container_detail",
        "vec_iterator<std::pair<CHashString,boost::container::static_vector<std::pair<PlayerId_Steam,int>,5>> *,1>",
        "vec_iterator<std::pair<CHashString,boost::container::static_vector<std::pair<PlayerId_Steam,int>,5>> *,1>"
    ]);
}

#[test]
fn test_signature_143() {
    let result = parse_signature("bool __fastcall CVehicleActionVolume::IsType");
    assert_eq!(result, vec![
        "CVehicleActionVolume",
        "IsType"
    ]);
}

#[test]
fn test_signature_144() {
    let result = parse_signature("bool __fastcall boost::tuples::detail::lt<boost::tuples::cons<CHashString,boost::tuples::cons<unsigned __int64,boost::tuples::null_type>>,boost::tuples::cons<CHashString,boost::tuples::cons<unsigned __int64,boost::tuples::null_type>>>");
    assert_eq!(result, vec![
        "boost",
        "tuples",
        "detail",
        "lt<boost::tuples::cons<CHashString,boost::tuples::cons<unsigned __int64,boost::tuples::null_type>>,boost::tuples::cons<CHashString,boost::tuples::cons<unsigned __int64,boost::tuples::null_type>>>"
    ]);
}

#[test]
fn test_signature_145() {
    let result = parse_signature("__int64 __fastcall Minuint16");
    assert_eq!(result, vec![
        "Minuint16"
    ]);
}

#[test]
fn test_signature_146() {
    let result = parse_signature("void __fastcall std::allocator<NRevolution::SProvinceRecord>::deallocate");
    assert_eq!(result, vec![
        "std",
        "allocator<NRevolution::SProvinceRecord>",
        "deallocate"
    ]);
}

#[test]
fn test_signature_147() {
    let result = parse_signature("CSequenceObject2 **__fastcall std::_Uninit_move<CSequenceObject2 *,CSequenceObject2 *,CSequenceObject2 *>");
    assert_eq!(result, vec![
        "std",
        "_Uninit_move<CSequenceObject2 *,CSequenceObject2 *,CSequenceObject2 *>"
    ]);
}

#[test]
fn test_signature_148() {
    let result = parse_signature("unsigned __int64 __fastcall std::vector<NRevolution::SPriestRecord>::_Grow_to");
    assert_eq!(result, vec![
        "std",
        "vector<NRevolution::SPriestRecord>",
        "_Grow_to"
    ]);
}

#[test]
fn test_signature_149() {
    let result = parse_signature("std::_Vector_iterator<std::_Vector_val<CSequenceObject2::SSeqAnimLayerKey2> > *__fastcall std::vector<CSequenceObject2::SSeqAnimLayerKey2>::end");
    assert_eq!(result, vec![
        "std",
        "vector<CSequenceObject2::SSeqAnimLayerKey2>",
        "end"
    ]);
}

#[test]
fn test_signature_150() {
    let result = parse_signature("void __fastcall std::_Cons_val<std::allocator<CSequenceObject2::SSeqPropertySegment>,CSequenceObject2::SSeqPropertySegment,CSequenceObject2::SSeqPropertySegment const &>");
    assert_eq!(result, vec![
        "std",
        "_Cons_val<std::allocator<CSequenceObject2::SSeqPropertySegment>,CSequenceObject2::SSeqPropertySegment,CSequenceObject2::SSeqPropertySegment const &>"
    ]);
}

#[test]
fn test_signature_151() {
    let result = parse_signature("void __fastcall std::tr1::function<void (CCallContext &)>::function<void (CCallContext &)>");
    assert_eq!(result, vec![
        "std",
        "tr1",
        "function<void (CCallContext &)>",
        "function<void (CCallContext &)>"
    ]);
}

#[test]
fn test_signature_152() {
    let result = parse_signature("const CHashString *__fastcall CCompoundBaseRule::GetClassHash");
    assert_eq!(result, vec![
        "CCompoundBaseRule",
        "GetClassHash"
    ]);
}

#[test]
fn test_signature_153() {
    let result = parse_signature("TaskQueue::CTask<enum NOnline::EErrorCode> *__fastcall NSaveLoad::CLocalDataStore::PersistAsync");
    assert_eq!(result, vec![
        "NSaveLoad",
        "CLocalDataStore",
        "PersistAsync"
    ]);
}

#[test]
fn test_signature_154() {
    let result = parse_signature("bool __fastcall TreeSortingPredicate");
    assert_eq!(result, vec![
        "TreeSortingPredicate"
    ]);
}

#[test]
fn test_signature_155() {
    let result = parse_signature("std::_Unique_ptr_base<CLeaderboardStrategy<CAsynchronousBragEntry<float>,float>,std::default_delete<CLeaderboardStrategy<CAsynchronousBragEntry<float>,float> >,1> *__fastcall std::_Unique_ptr_base<CLeaderboardStrategy<CAsynchronousBragEntry<float>,float>,std::default_delete<CLeaderboardStrategy<CAsynchronousBragEntry<float>,float>>,1>::get_deleter");
    assert_eq!(result, vec![
        "std",
        "_Unique_ptr_base<CLeaderboardStrategy<CAsynchronousBragEntry<float>,float>,std::default_delete<CLeaderboardStrategy<CAsynchronousBragEntry<float>,float>>,1>",
        "get_deleter"
    ]);
}

#[test]
fn test_signature_156() {
    let result = parse_signature("const CPfxShapeSensor::SShapeInfo *__fastcall hkAddByteOffsetConst<CPfxShapeSensor::SShapeInfo>");
    assert_eq!(result, vec![
        "hkAddByteOffsetConst<CPfxShapeSensor::SShapeInfo>"
    ]);
}

#[test]
fn test_signature_157() {
    let result = parse_signature("void __fastcall std::unique_ptr<CAsynchronousBragStrategy<__int64>>::unique_ptr<CAsynchronousBragStrategy<__int64>>");
    assert_eq!(result, vec![
        "std",
        "unique_ptr<CAsynchronousBragStrategy<__int64>>",
        "unique_ptr<CAsynchronousBragStrategy<__int64>>"
    ]);
}

#[test]
fn test_signature_158() {
    let result = parse_signature("void __fastcall std::_Distance<CCommSocialUI::SSocialEntry *,__int64>");
    assert_eq!(result, vec![
        "std",
        "_Distance<CCommSocialUI::SSocialEntry *,__int64>"
    ]);
}

#[test]
fn test_signature_159() {
    let result = parse_signature("std::_List_iterator<std::_List_val<std::pair<NRevolution::SRegionRecord const * const,std::string >> > *__fastcall std::list<std::pair<NRevolution::SRegionRecord const * const,std::string>>::begin");
    assert_eq!(result, vec![
        "std",
        "list<std::pair<NRevolution::SRegionRecord const * const,std::string>>",
        "begin"
    ]);
}

#[test]
fn test_signature_160() {
    let result = parse_signature("void __fastcall boost::tuples::cons<CHashString,boost::tuples::cons<CHashString,boost::tuples::cons<CHashString,boost::tuples::null_type>>>::~cons<CHashString,boost::tuples::cons<CHashString,boost::tuples::cons<CHashString,boost::tuples::null_type>>>");
    assert_eq!(result, vec![
        "boost",
        "tuples",
        "cons<CHashString,boost::tuples::cons<CHashString,boost::tuples::cons<CHashString,boost::tuples::null_type>>>",
        "~cons<CHashString,boost::tuples::cons<CHashString,boost::tuples::cons<CHashString,boost::tuples::null_type>>>"
    ]);
}

#[test]
fn test_signature_161() {
    let result = parse_signature("void __fastcall std::tr1::_Callable_obj<`anonymous namespace'::_lambda61_,0>::_Callable_obj<`anonymous namespace'::_lambda61_,0>");
    assert_eq!(result, vec![
        "std",
        "tr1",
        "_Callable_obj<`anonymous namespace'::_lambda61_,0>",
        "_Callable_obj<`anonymous namespace'::_lambda61_,0>"
    ]);
}

#[test]
fn test_signature_162() {
    let result = parse_signature("void __fastcall boost::container::vector<CHashString,std::allocator<CHashString>>::priv_forward_range_insert_new_allocation<boost::container::container_detail::insert_copy_proxy<std::allocator<CHashString>,CHashString *>>");
    assert_eq!(result, vec![
        "boost",
        "container",
        "vector<CHashString,std::allocator<CHashString>>",
        "priv_forward_range_insert_new_allocation<boost::container::container_detail::insert_copy_proxy<std::allocator<CHashString>,CHashString *>>"
    ]);
}

#[test]
fn test_signature_163() {
    let result = parse_signature("CPauseUI *__fastcall CPauseUI::`vector deleting destructor'");
    assert_eq!(result, vec![
        "CPauseUI",
        "`vector deleting destructor'"
    ]);
}

#[test]
fn test_signature_164() {
    let result = parse_signature("float __fastcall CHUDUI::GetHealthBarValue");
    assert_eq!(result, vec![
        "CHUDUI",
        "GetHealthBarValue"
    ]);
}

#[test]
fn test_signature_165() {
    let result = parse_signature("void __fastcall ValueTypeTrait<3825342993>::DefaultValue");
    assert_eq!(result, vec![
        "ValueTypeTrait<3825342993>",
        "DefaultValue"
    ]);
}

#[test]
fn test_signature_166() {
    let result = parse_signature("void __fastcall std::_Vector_const_iterator<std::_Vector_val<CVehicle::SProcedurallyAnimatedPartInfo>>::_Vector_const_iterator<std::_Vector_val<CVehicle::SProcedurallyAnimatedPartInfo>>");
    assert_eq!(result, vec![
        "std",
        "_Vector_const_iterator<std::_Vector_val<CVehicle::SProcedurallyAnimatedPartInfo>>",
        "_Vector_const_iterator<std::_Vector_val<CVehicle::SProcedurallyAnimatedPartInfo>>"
    ]);
}

#[test]
fn test_signature_167() {
    let result = parse_signature("void __fastcall ValueTypeTrait<1300093458>::DefaultValue");
    assert_eq!(result, vec![
        "ValueTypeTrait<1300093458>",
        "DefaultValue"
    ]);
}

#[test]
fn test_signature_168() {
    let result = parse_signature("void __fastcall ExtractValueHelper<VehicleValueCache,319901926>::ExtractValue");
    assert_eq!(result, vec![
        "ExtractValueHelper<VehicleValueCache,319901926>",
        "ExtractValue"
    ]);
}

#[test]
fn test_signature_169() {
    let result = parse_signature("void __fastcall boost::detail::function::functor_manager_common<`anonymous namespace'::_lambda174_>::manage_small");
    assert_eq!(result, vec![
        "boost",
        "detail",
        "function",
        "functor_manager_common<`anonymous namespace'::_lambda174_>",
        "manage_small"
    ]);
}

#[test]
fn test_signature_170() {
    let result = parse_signature("void __fastcall boost::detail::addr_impl_ref<`anonymous namespace'::_lambda189_>::addr_impl_ref<`anonymous namespace'::_lambda189_>");
    assert_eq!(result, vec![
        "boost",
        "detail",
        "addr_impl_ref<`anonymous namespace'::_lambda189_>",
        "addr_impl_ref<`anonymous namespace'::_lambda189_>"
    ]);
}

#[test]
fn test_signature_171() {
    let result = parse_signature("void __fastcall NVehicle_Hidden::DebugDraw_BoatStability");
    assert_eq!(result, vec![
        "NVehicle_Hidden",
        "DebugDraw_BoatStability"
    ]);
}

#[test]
fn test_signature_172() {
    let result = parse_signature("void __fastcall VehicleValueCache::ExtractValue<2289800969>");
    assert_eq!(result, vec![
        "VehicleValueCache",
        "ExtractValue<2289800969>"
    ]);
}

#[test]
fn test_signature_173() {
    let result = parse_signature("<lambda261> *__fastcall boost::addressof<`anonymous namespace'::_lambda261_>");
    assert_eq!(result, vec![
        "boost",
        "addressof<`anonymous namespace'::_lambda261_>"
    ]);
}

#[test]
fn test_signature_174() {
    let result = parse_signature("<lambda39> *__fastcall std::_For_each<dynamo::vm::executable *,`anonymous namespace'::_lambda39_>");
    assert_eq!(result, vec![
        "std",
        "_For_each<dynamo::vm::executable *,`anonymous namespace'::_lambda39_>"
    ]);
}

#[test]
fn test_signature_175() {
    let result = parse_signature("char __fastcall boost::detail::function::basic_vtable0<float>::assign_to<`anonymous namespace'::_lambda273_>");
    assert_eq!(result, vec![
        "boost",
        "detail",
        "function",
        "basic_vtable0<float>",
        "assign_to<`anonymous namespace'::_lambda273_>"
    ]);
}

#[test]
fn test_signature_176() {
    let result = parse_signature("void __fastcall boost::detail::function::functor_manager<`anonymous namespace'::_lambda308_>::manage");
    assert_eq!(result, vec![
        "boost",
        "detail",
        "function",
        "functor_manager<`anonymous namespace'::_lambda308_>",
        "manage"
    ]);
}

#[test]
fn test_signature_177() {
    let result = parse_signature("void __fastcall std::_Uninitialized_fill_n<std::_List_iterator<std::_List_val<std::pair<unsigned int const,CScaleformTranslator::ArabicCharSwap>>> *,unsigned __int64,std::_List_iterator<std::_List_val<std::pair<unsigned int const,CScaleformTranslator::ArabicCharSwap>>>,std::allocator<std::_List_iterator<std::_List_val<std::pair<unsigned int const,CScaleformTranslator::ArabicCharSwap>>>>>");
    assert_eq!(result, vec![
        "std",
        "_Uninitialized_fill_n<std::_List_iterator<std::_List_val<std::pair<unsigned int const,CScaleformTranslator::ArabicCharSwap>>> *,unsigned __int64,std::_List_iterator<std::_List_val<std::pair<unsigned int const,CScaleformTranslator::ArabicCharSwap>>>,std::allocator<std::_List_iterator<std::_List_val<std::pair<unsigned int const,CScaleformTranslator::ArabicCharSwap>>>>>"
    ]);
}

#[test]
fn test_signature_178() {
    let result = parse_signature("void __fastcall HookupValueGetterImpl<2588563556>");
    assert_eq!(result, vec![
        "HookupValueGetterImpl<2588563556>"
    ]);
}

#[test]
fn test_signature_179() {
    let result = parse_signature("void __fastcall HookupValueGetterImpl<3430925030>");
    assert_eq!(result, vec![
        "HookupValueGetterImpl<3430925030>"
    ]);
}

#[test]
fn test_signature_180() {
    let result = parse_signature("void __fastcall boost::function0<float>::assign_to<`anonymous namespace'::_lambda246_>");
    assert_eq!(result, vec![
        "boost",
        "function0<float>",
        "assign_to<`anonymous namespace'::_lambda246_>"
    ]);
}

#[test]
fn test_signature_181() {
    let result = parse_signature("NEvent::CReceiveEvent *__fastcall std::map<std::string,NEvent::CReceiveEvent>::operator[]");
    assert_eq!(result, vec![
        "std",
        "map<std::string,NEvent::CReceiveEvent>",
        "operator[]"
    ]);
}

#[test]
fn test_signature_182() {
    let result = parse_signature("void __fastcall HookupDynamoConstant<2290896295>");
    assert_eq!(result, vec![
        "HookupDynamoConstant<2290896295>"
    ]);
}

#[test]
fn test_signature_183() {
    let result = parse_signature("CHelicopter *__fastcall CHelicopter::`vector deleting destructor'");
    assert_eq!(result, vec![
        "CHelicopter",
        "`vector deleting destructor'"
    ]);
}

#[test]
fn test_signature_184() {
    let result = parse_signature("boost::container::container_detail::vector_alloc_holder<std::allocator<boost::container::container_detail::pair<CCallContext const *,boost::container::flat_map<int,SStoreItemInfo,std::less<int>,std::allocator<std::pair<int,SStoreItemInfo> > > *> >,boost::container::container_detail::integral_constant<unsigned int,1> > *__fastcall boost::container::container_detail::vector_alloc_holder<std::allocator<boost::container::container_detail::pair<CCallContext const *,boost::container::flat_map<int,SStoreItemInfo,std::less<int>,std::allocator<std::pair<int,SStoreItemInfo>>> *>>,boost::container::container_detail::integral_constant<unsigned int,1>>::alloc");
    assert_eq!(result, vec![
        "boost",
        "container",
        "container_detail",
        "vector_alloc_holder<std::allocator<boost::container::container_detail::pair<CCallContext const *,boost::container::flat_map<int,SStoreItemInfo,std::less<int>,std::allocator<std::pair<int,SStoreItemInfo>>> *>>,boost::container::container_detail::integral_constant<unsigned int,1>>",
        "alloc"
    ]);
}

#[test]
fn test_signature_185() {
    let result = parse_signature("void __fastcall boost::container::container_detail::scoped_destructor_n<std::allocator<boost::container::container_detail::pair<CCallContext const *,boost::shared_ptr<CSessionInfo_Steam>>>>::~scoped_destructor_n<std::allocator<boost::container::container_detail::pair<CCallContext const *,boost::shared_ptr<CSessionInfo_Steam>>>>");
    assert_eq!(result, vec![
        "boost",
        "container",
        "container_detail",
        "scoped_destructor_n<std::allocator<boost::container::container_detail::pair<CCallContext const *,boost::shared_ptr<CSessionInfo_Steam>>>>",
        "~scoped_destructor_n<std::allocator<boost::container::container_detail::pair<CCallContext const *,boost::shared_ptr<CSessionInfo_Steam>>>>"
    ]);
}

#[test]
fn test_signature_186() {
    let result = parse_signature("char __fastcall ReadPostPayload");
    assert_eq!(result, vec![
        "ReadPostPayload"
    ]);
}

#[test]
fn test_signature_187() {
    let result = parse_signature("float3 *__fastcall Texture2DArray<float3,6>::SampleLevel");
    assert_eq!(result, vec![
        "Texture2DArray<float3,6>",
        "SampleLevel"
    ]);
}

#[test]
fn test_signature_188() {
    let result = parse_signature("OSuite::TAtomObject<OSuite::ZOEntry> *__fastcall OSuite::TAtomObject<OSuite::ZOEntry>::`scalar deleting destructor'");
    assert_eq!(result, vec![
        "OSuite",
        "TAtomObject<OSuite::ZOEntry>",
        "`scalar deleting destructor'"
    ]);
}

#[test]
fn test_signature_189() {
    let result = parse_signature("OSuite::ManualRefCount<OSuite::IHttpRequest> *__fastcall OSuite::ZHttpRequestManager::CreateHttpRequest");
    assert_eq!(result, vec![
        "OSuite",
        "ZHttpRequestManager",
        "CreateHttpRequest"
    ]);
}

#[test]
fn test_signature_190() {
    let result = parse_signature("OSuite::TOrderedMap<OSuite::ZString,OSuitePrivate::ZNotificationRequest *,OSuite::TOperatorComparer<OSuite::ZString> >::ZIterator *__fastcall OSuite::TOrderedMap<OSuite::ZString,OSuitePrivate::ZNotificationRequest *,OSuite::TOperatorComparer<OSuite::ZString>>::Iterator");
    assert_eq!(result, vec![
        "OSuite",
        "TOrderedMap<OSuite::ZString,OSuitePrivate::ZNotificationRequest *,OSuite::TOperatorComparer<OSuite::ZString>>",
        "Iterator"
    ]);
}

#[test]
fn test_signature_191() {
    let result = parse_signature("void *__fastcall OSuite::ZObject::realloc");
    assert_eq!(result, vec![
        "OSuite",
        "ZObject",
        "realloc"
    ]);
}

#[test]
fn test_signature_192() {
    let result = parse_signature("__int64 __fastcall Z_do_client_connect");
    assert_eq!(result, vec![
        "Z_do_client_connect"
    ]);
}

#[test]
fn test_signature_193() {
    let result = parse_signature("const hkClass *__fastcall hkOstream::getClassType");
    assert_eq!(result, vec![
        "hkOstream",
        "getClassType"
    ]);
}

#[test]
fn test_signature_194() {
    let result = parse_signature("void __fastcall hkArrayBase<hkFreeListArrayElement<hkTaskQueue::GraphInfo>>::~hkArrayBase<hkFreeListArrayElement<hkTaskQueue::GraphInfo>>");
    assert_eq!(result, vec![
        "hkArrayBase<hkFreeListArrayElement<hkTaskQueue::GraphInfo>>",
        "~hkArrayBase<hkFreeListArrayElement<hkTaskQueue::GraphInfo>>"
    ]);
}

#[test]
fn test_signature_195() {
    let result = parse_signature("void __fastcall hkTypeInfo::finishLoadedObject");
    assert_eq!(result, vec![
        "hkTypeInfo",
        "finishLoadedObject"
    ]);
}

#[test]
fn test_signature_196() {
    let result = parse_signature("hkStdioStreamReader *__fastcall hkStdioStreamReader::`vector deleting destructor'");
    assert_eq!(result, vec![
        "hkStdioStreamReader",
        "`vector deleting destructor'"
    ]);
}

#[test]
fn test_signature_197() {
    let result = parse_signature("void __fastcall hkMultipleVertexBuffer::hkMultipleVertexBuffer");
    assert_eq!(result, vec![
        "hkMultipleVertexBuffer",
        "hkMultipleVertexBuffer"
    ]);
}

#[test]
fn test_signature_198() {
    let result = parse_signature("__int64 __fastcall hkArrayBase<hkRefPtr<hkxSkinBinding>>::getCapacity");
    assert_eq!(result, vec![
        "hkArrayBase<hkRefPtr<hkxSkinBinding>>",
        "getCapacity"
    ]);
}

#[test]
fn test_signature_199() {
    let result = parse_signature("void __fastcall s_setOffset");
    assert_eq!(result, vec![
        "s_setOffset"
    ]);
}

#[test]
fn test_signature_200() {
    let result = parse_signature("__int64 __fastcall hkDataObject::getMemberIterator");
    assert_eq!(result, vec![
        "hkDataObject",
        "getMemberIterator"
    ]);
}

#[test]
fn test_signature_201() {
    let result = parse_signature("void __fastcall hkArrayBase<hkDataClassImpl *>::pushBackUnchecked");
    assert_eq!(result, vec![
        "hkArrayBase<hkDataClassImpl *>",
        "pushBackUnchecked"
    ]);
}

#[test]
fn test_signature_202() {
    let result = parse_signature("hkResult *__fastcall hkDebugDisplayProcess::displayTriangle");
    assert_eq!(result, vec![
        "hkDebugDisplayProcess",
        "displayTriangle"
    ]);
}

#[test]
fn test_signature_203() {
    let result = parse_signature("void __fastcall hkArrayBase<hkArray<hkHandle<unsigned int,268435455,hkcdPlanarGeometryPrimitives::PlaneIdDiscriminant>,hkContainerHeapAllocator>>::hkArrayBase<hkArray<hkHandle<unsigned int,268435455,hkcdPlanarGeometryPrimitives::PlaneIdDiscriminant>,hkContainerHeapAllocator>>");
    assert_eq!(result, vec![
        "hkArrayBase<hkArray<hkHandle<unsigned int,268435455,hkcdPlanarGeometryPrimitives::PlaneIdDiscriminant>,hkContainerHeapAllocator>>",
        "hkArrayBase<hkArray<hkHandle<unsigned int,268435455,hkcdPlanarGeometryPrimitives::PlaneIdDiscriminant>,hkContainerHeapAllocator>>"
    ]);
}

#[test]
fn test_signature_204() {
    let result = parse_signature("void __fastcall hkFixedArray<enum hkcdPlanarGeometryPredicates::Orientation>::hkFixedArray<enum hkcdPlanarGeometryPredicates::Orientation>");
    assert_eq!(result, vec![
        "hkFixedArray<enum hkcdPlanarGeometryPredicates::Orientation>",
        "hkFixedArray<enum hkcdPlanarGeometryPredicates::Orientation>"
    ]);
}

#[test]
fn test_signature_205() {
    let result = parse_signature("void __fastcall hkgpConvexHull::getOrientedBoundingBox");
    assert_eq!(result, vec![
        "hkgpConvexHull",
        "getOrientedBoundingBox"
    ]);
}

#[test]
fn test_signature_206() {
    let result = parse_signature("hkGeometryProcessing::ConstFunction<hkGeometryProcessing::IFunction<hkVector4f,float> > *__fastcall hkGeometryProcessing::ConstFunction<hkGeometryProcessing::IFunction<hkVector4f,float>>::`scalar deleting destructor'");
    assert_eq!(result, vec![
        "hkGeometryProcessing",
        "ConstFunction<hkGeometryProcessing::IFunction<hkVector4f,float>>",
        "`scalar deleting destructor'"
    ]);
}

#[test]
fn test_signature_207() {
    let result = parse_signature("void __fastcall hkcdTreeQueries<hkcdTreeQueriesStacks::Dynamic,64,0>::ClosestFromPointWrapper<NearestFeaturePolicy>::ClosestFromPointWrapper<NearestFeaturePolicy>");
    assert_eq!(result, vec![
        "hkcdTreeQueries<hkcdTreeQueriesStacks::Dynamic,64,0>",
        "ClosestFromPointWrapper<NearestFeaturePolicy>",
        "ClosestFromPointWrapper<NearestFeaturePolicy>"
    ]);
}

#[test]
fn test_signature_208() {
    let result = parse_signature("__int64 __fastcall hkMath::min2_int_int__27");
    assert_eq!(result, vec![
        "hkMath",
        "min2_int_int__27"
    ]);
}

#[test]
fn test_signature_209() {
    let result = parse_signature("hkcdTreeQueries<hkcdTreeQueriesStacks::Dynamic,64,0>::SlotPair<hkcdStaticTree::DefaultTreeStorage6,hkcdStaticTree::DefaultTreeStorage6> *__fastcall hkArrayBase<hkcdTreeQueries<hkcdTreeQueriesStacks::Dynamic,64,0>::SlotPair<hkcdStaticTree::DefaultTreeStorage6,hkcdStaticTree::DefaultTreeStorage6>>::_expandOne");
    assert_eq!(result, vec![
        "hkArrayBase<hkcdTreeQueries<hkcdTreeQueriesStacks::Dynamic,64,0>::SlotPair<hkcdStaticTree::DefaultTreeStorage6,hkcdStaticTree::DefaultTreeStorage6>>",
        "_expandOne"
    ]);
}

#[test]
fn test_signature_210() {
    let result = parse_signature("void __fastcall hkpWheelConstraintData::Atoms::Atoms");
    assert_eq!(result, vec![
        "hkpWheelConstraintData",
        "Atoms",
        "Atoms"
    ]);
}

#[test]
fn test_signature_211() {
    let result = parse_signature("__int64 __fastcall hkcdTreeQueries<hkcdTreeQueriesStacks::Dynamic,64,0>::IfFilterNode<hknpCompressedMeshShapeInternals::RayCastQuery<1>,hkcdStaticTree::Tree<hkcdStaticTree::DynamicStorage5>::NodeContext>::call<hkcdStaticTree::Tree<hkcdStaticTree::DynamicStorage5>::NodeContext>");
    assert_eq!(result, vec![
        "hkcdTreeQueries<hkcdTreeQueriesStacks::Dynamic,64,0>",
        "IfFilterNode<hknpCompressedMeshShapeInternals::RayCastQuery<1>,hkcdStaticTree::Tree<hkcdStaticTree::DynamicStorage5>::NodeContext>",
        "call<hkcdStaticTree::Tree<hkcdStaticTree::DynamicStorage5>::NodeContext>"
    ]);
}

#[test]
fn test_signature_212() {
    let result = parse_signature("__int64 __fastcall hkcdTreeQueries<hkcdTreeQueriesStacks::Dynamic,64,0>::IfBeginFront<hkcdTreeQueries<hkcdTreeQueriesStacks::Dynamic,64,0>::AabbOverlapsNoEarlyExitWrapper<hknpCompoundShapeInternals<hknpStaticCompoundShapeInternals>::AabbOverlaps<1,hkArray<unsigned int,hkContainerHeapAllocator>>>>::Default::call<hknpStaticCompoundShapeTree>");
    assert_eq!(result, vec![
        "hkcdTreeQueries<hkcdTreeQueriesStacks::Dynamic,64,0>",
        "IfBeginFront<hkcdTreeQueries<hkcdTreeQueriesStacks::Dynamic,64,0>::AabbOverlapsNoEarlyExitWrapper<hknpCompoundShapeInternals<hknpStaticCompoundShapeInternals>::AabbOverlaps<1,hkArray<unsigned int,hkContainerHeapAllocator>>>>",
        "Default",
        "call<hknpStaticCompoundShapeTree>"
    ]);
}

#[test]
fn test_signature_213() {
    let result = parse_signature("void __fastcall hkSignal1<unsigned char>::_fire");
    assert_eq!(result, vec![
        "hkSignal1<unsigned char>",
        "_fire"
    ]);
}

#[test]
fn test_signature_214() {
    let result = parse_signature("void __fastcall hkBitFieldBase<hkBitFieldStorage<hkArray<unsigned int,hkContainerHeapAllocator>>>::Iterator::getNext");
    assert_eq!(result, vec![
        "hkBitFieldBase<hkBitFieldStorage<hkArray<unsigned int,hkContainerHeapAllocator>>>",
        "Iterator",
        "getNext"
    ]);
}

#[test]
fn test_signature_215() {
    let result = parse_signature("void __fastcall hknpSingleCellSpaceSplitter::operator delete");
    assert_eq!(result, vec![
        "hknpSingleCellSpaceSplitter",
        "operator delete"
    ]);
}

#[test]
fn test_signature_216() {
    let result = parse_signature("void __fastcall finishLoadedObjecthknpVehicleInstance");
    assert_eq!(result, vec![
        "finishLoadedObjecthknpVehicleInstance"
    ]);
}

#[test]
fn test_signature_217() {
    let result = parse_signature("void __fastcall hkFreeListArray<hknpMaterial,hknpMaterialId,8,hknpMaterial::FreeListArrayOperations>::release");
    assert_eq!(result, vec![
        "hkFreeListArray<hknpMaterial,hknpMaterialId,8,hknpMaterial::FreeListArrayOperations>",
        "release"
    ]);
}

#[test]
fn test_signature_218() {
    let result = parse_signature("void __fastcall hknpConstraintForceExceededEvent::hknpConstraintForceExceededEvent");
    assert_eq!(result, vec![
        "hknpConstraintForceExceededEvent",
        "hknpConstraintForceExceededEvent"
    ]);
}

#[test]
fn test_signature_219() {
    let result = parse_signature("void __fastcall hknpDecoratorShape::queryAabbImpl");
    assert_eq!(result, vec![
        "hknpDecoratorShape",
        "queryAabbImpl"
    ]);
}

#[test]
fn test_signature_220() {
    let result = parse_signature("bool __fastcall hkHandle<unsigned short,65535,hknpImmediateConstraintIdDiscriminant>::operator==");
    assert_eq!(result, vec![
        "hkHandle<unsigned short,65535,hknpImmediateConstraintIdDiscriminant>",
        "operator=="
    ]);
}

#[test]
fn test_signature_221() {
    let result = parse_signature("hkBitFieldInplaceStorage<8> *__fastcall hkBitFieldInplaceStorage<8>::operator=");
    assert_eq!(result, vec![
        "hkBitFieldInplaceStorage<8>",
        "operator="
    ]);
}

#[test]
fn test_signature_222() {
    let result = parse_signature("hkArray<int,hkContainerHeapAllocator> *__fastcall hkArray<int,hkContainerHeapAllocator>::operator=");
    assert_eq!(result, vec![
        "hkArray<int,hkContainerHeapAllocator>",
        "operator="
    ]);
}

#[test]
fn test_signature_223() {
    let result = parse_signature("void __fastcall hkaiPointCloudSilhouetteGenerator::update");
    assert_eq!(result, vec![
        "hkaiPointCloudSilhouetteGenerator",
        "update"
    ]);
}

#[test]
fn test_signature_224() {
    let result = parse_signature("void __fastcall hkRefPtr<hkaiWorld>::~hkRefPtr<hkaiWorld>");
    assert_eq!(result, vec![
        "hkRefPtr<hkaiWorld>",
        "~hkRefPtr<hkaiWorld>"
    ]);
}

#[test]
fn test_signature_225() {
    let result = parse_signature("hkSimdFloat32 *__fastcall hkcdPointSegmentDistanceSquared_13");
    assert_eq!(result, vec![
        "hkcdPointSegmentDistanceSquared_13"
    ]);
}

#[test]
fn test_signature_226() {
    let result = parse_signature("void __fastcall hkaiNavMesh::checkDeterminism");
    assert_eq!(result, vec![
        "hkaiNavMesh",
        "checkDeterminism"
    ]);
}

#[test]
fn test_signature_227() {
    let result = parse_signature("hkArray<hkaiStreamingSet::GraphConnection,hkContainerHeapAllocator> *__fastcall hkArray<hkaiStreamingSet::GraphConnection,hkContainerHeapAllocator>::operator=");
    assert_eq!(result, vec![
        "hkArray<hkaiStreamingSet::GraphConnection,hkContainerHeapAllocator>",
        "operator="
    ]);
}

#[test]
fn test_signature_228() {
    let result = parse_signature("hkaiNavMeshEdgeLabelsViewer *__fastcall hkaiNavMeshEdgeLabelsViewer::`scalar deleting destructor'");
    assert_eq!(result, vec![
        "hkaiNavMeshEdgeLabelsViewer",
        "`scalar deleting destructor'"
    ]);
}

#[test]
fn test_signature_229() {
    let result = parse_signature("void __fastcall hkQueue<int>::~hkQueue<int>");
    assert_eq!(result, vec![
        "hkQueue<int>",
        "~hkQueue<int>"
    ]);
}

#[test]
fn test_signature_230() {
    let result = parse_signature("hkResult *__fastcall hkArrayBase<hkaiNavMeshSimplificationUtils::SegmentIdxPid>::_reserve");
    assert_eq!(result, vec![
        "hkArrayBase<hkaiNavMeshSimplificationUtils::SegmentIdxPid>",
        "_reserve"
    ]);
}

#[test]
fn test_signature_231() {
    let result = parse_signature("void __fastcall hkArrayBase<hkaiGeometryToEdgeGeometryConverter::TriangleArea>::hkArrayBase<hkaiGeometryToEdgeGeometryConverter::TriangleArea>");
    assert_eq!(result, vec![
        "hkArrayBase<hkaiGeometryToEdgeGeometryConverter::TriangleArea>",
        "hkArrayBase<hkaiGeometryToEdgeGeometryConverter::TriangleArea>"
    ]);
}

#[test]
fn test_signature_232() {
    let result = parse_signature("__int64 __fastcall hkaiClimbUpUtils::getNumGrabSegments");
    assert_eq!(result, vec![
        "hkaiClimbUpUtils",
        "getNumGrabSegments"
    ]);
}

#[test]
fn test_signature_233() {
    let result = parse_signature("void __fastcall hkArrayBase<hkndBreakableBody>::clear");
    assert_eq!(result, vec![
        "hkArrayBase<hkndBreakableBody>",
        "clear"
    ]);
}

#[test]
fn test_signature_234() {
    let result = parse_signature("void __fastcall hkndHierarchyInstance::clone");
    assert_eq!(result, vec![
        "hkndHierarchyInstance",
        "clone"
    ]);
}

#[test]
fn test_signature_235() {
    let result = parse_signature("void *__fastcall hkndFxDebrisFracture::operator new");
    assert_eq!(result, vec![
        "hkndFxDebrisFracture",
        "operator new"
    ]);
}

#[test]
fn test_signature_236() {
    let result = parse_signature("void __fastcall hkndCompoundShapeImpl::GatherShapes::operator()");
    assert_eq!(result, vec![
        "hkndCompoundShapeImpl",
        "GatherShapes",
        "operator()"
    ]);
}

#[test]
fn test_signature_237() {
    let result = parse_signature("void __fastcall hkArrayBase<hkRefPtr<hkndSplitByPhysicsIslandsUtil::SplitByIslandsResult>>::_setSize");
    assert_eq!(result, vec![
        "hkArrayBase<hkRefPtr<hkndSplitByPhysicsIslandsUtil::SplitByIslandsResult>>",
        "_setSize"
    ]);
}

#[test]
fn test_signature_238() {
    let result = parse_signature("_BOOL8 __fastcall hkndSolidToGeomConverterImpl::VRepGeom::triangleFlipped");
    assert_eq!(result, vec![
        "hkndSolidToGeomConverterImpl",
        "VRepGeom",
        "triangleFlipped"
    ]);
}

#[test]
fn test_signature_239() {
    let result = parse_signature("void __fastcall hkArrayUtil::constructWithArray<hkRefPtr<hkndFractureEngine::Shape>>");
    assert_eq!(result, vec![
        "hkArrayUtil",
        "constructWithArray<hkRefPtr<hkndFractureEngine::Shape>>"
    ]);
}

#[test]
fn test_signature_240() {
    let result = parse_signature("char __fastcall hkndDecorateFractureFaceActionImpl::PartialLine::intersectWithObb");
    assert_eq!(result, vec![
        "hkndDecorateFractureFaceActionImpl",
        "PartialLine",
        "intersectWithObb"
    ]);
}

#[test]
fn test_signature_241() {
    let result = parse_signature("hkndDecorationGraph::DecorationInstance *__fastcall hkndDecorationGraph::addNewDecorationInstance");
    assert_eq!(result, vec![
        "hkndDecorationGraph",
        "addNewDecorationInstance"
    ]);
}

#[test]
fn test_signature_242() {
    let result = parse_signature("void __fastcall Graphics::SetVertexProgramConstantBufferNoMutex");
    assert_eq!(result, vec![
        "Graphics",
        "SetVertexProgramConstantBufferNoMutex"
    ]);
}

#[test]
fn test_signature_243() {
    let result = parse_signature("__int64 __fastcall Scaleform::GFx::ActionControl::GetActionFlags");
    assert_eq!(result, vec![
        "Scaleform",
        "GFx",
        "ActionControl",
        "GetActionFlags"
    ]);
}

#[test]
fn test_signature_244() {
    let result = parse_signature("void __fastcall Scaleform::ArrayDataBase<unsigned char,Scaleform::AllocatorGH_POD<unsigned char,2>,Scaleform::ArrayDefaultPolicy>::ResizeNoConstruct");
    assert_eq!(result, vec![
        "Scaleform",
        "ArrayDataBase<unsigned char,Scaleform::AllocatorGH_POD<unsigned char,2>,Scaleform::ArrayDefaultPolicy>",
        "ResizeNoConstruct"
    ]);
}

#[test]
fn test_signature_245() {
    let result = parse_signature("void __fastcall Scaleform::Render::ContextImpl::ContextData_ImplMixin<Scaleform::Render::TreeText::NodeData,Scaleform::Render::TreeNode::NodeData>::ConstructCopy");
    assert_eq!(result, vec![
        "Scaleform",
        "Render",
        "ContextImpl",
        "ContextData_ImplMixin<Scaleform::Render::TreeText::NodeData,Scaleform::Render::TreeNode::NodeData>",
        "ConstructCopy"
    ]);
}

#[test]
fn test_signature_246() {
    let result = parse_signature("const Scaleform::Render::MatrixPoolImpl::HMatrix *__fastcall Scaleform::ArrayBase<Scaleform::ArrayData<Scaleform::Render::MatrixPoolImpl::HMatrix,Scaleform::AllocatorLH<Scaleform::Render::MatrixPoolImpl::HMatrix,2>,Scaleform::ArrayDefaultPolicy>>::operator[]");
    assert_eq!(result, vec![
        "Scaleform",
        "ArrayBase<Scaleform::ArrayData<Scaleform::Render::MatrixPoolImpl::HMatrix,Scaleform::AllocatorLH<Scaleform::Render::MatrixPoolImpl::HMatrix,2>,Scaleform::ArrayDefaultPolicy>>",
        "operator[]"
    ]);
}

#[test]
fn test_signature_247() {
    let result = parse_signature("void __fastcall Scaleform::HashsetCachedNodeEntry<Scaleform::StringLH_HashNode<Scaleform::GFx::FontMap::MapEntry,Scaleform::String::NoCaseHashFunctor>,Scaleform::StringLH_HashNode<Scaleform::GFx::FontMap::MapEntry,Scaleform::String::NoCaseHashFunctor>::NodeHashF>::Clear");
    assert_eq!(result, vec![
        "Scaleform",
        "HashsetCachedNodeEntry<Scaleform::StringLH_HashNode<Scaleform::GFx::FontMap::MapEntry,Scaleform::String::NoCaseHashFunctor>,Scaleform::StringLH_HashNode<Scaleform::GFx::FontMap::MapEntry,Scaleform::String::NoCaseHashFunctor>::NodeHashF>",
        "Clear"
    ]);
}

#[test]
fn test_signature_248() {
    let result = parse_signature("void __fastcall Scaleform::ArrayDataBase<Scaleform::GFx::ImportData::Symbol,Scaleform::AllocatorLH<Scaleform::GFx::ImportData::Symbol,2>,Scaleform::ArrayDefaultPolicy>::Reserve");
    assert_eq!(result, vec![
        "Scaleform",
        "ArrayDataBase<Scaleform::GFx::ImportData::Symbol,Scaleform::AllocatorLH<Scaleform::GFx::ImportData::Symbol,2>,Scaleform::ArrayDefaultPolicy>",
        "Reserve"
    ]);
}

#[test]
fn test_signature_249() {
    let result = parse_signature("Scaleform::RefCountBaseStatImpl<Scaleform::RefCountImpl,324> *__fastcall Scaleform::RefCountBaseStatImpl<Scaleform::RefCountImpl,324>::`vector deleting destructor'");
    assert_eq!(result, vec![
        "Scaleform",
        "RefCountBaseStatImpl<Scaleform::RefCountImpl,324>",
        "`vector deleting destructor'"
    ]);
}

#[test]
fn test_signature_250() {
    let result = parse_signature("bool __fastcall Scaleform::Ptr<Scaleform::GFx::MovieDefImpl::BindTaskData>::operator==");
    assert_eq!(result, vec![
        "Scaleform",
        "Ptr<Scaleform::GFx::MovieDefImpl::BindTaskData>",
        "operator=="
    ]);
}

#[test]
fn test_signature_251() {
    let result = parse_signature("const Scaleform::Ptr<Scaleform::GFx::Resource> *__fastcall Scaleform::Ptr<Scaleform::GFx::Resource>::operator=<Scaleform::GFx::ImageResource>");
    assert_eq!(result, vec![
        "Scaleform",
        "Ptr<Scaleform::GFx::Resource>",
        "operator=<Scaleform::GFx::ImageResource>"
    ]);
}

#[test]
fn test_signature_252() {
    let result = parse_signature("unsigned __int64 __fastcall Scaleform::HashSetBase<Scaleform::GFx::FontManager::NodePtr,Scaleform::GFx::FontManager::NodePtrHashOp,Scaleform::GFx::FontManager::NodePtrHashOp,Scaleform::AllocatorLH<Scaleform::GFx::FontManager::NodePtr,2>,Scaleform::HashsetCachedEntry<Scaleform::GFx::FontManager::NodePtr,Scaleform::GFx::FontManager::NodePtrHashOp>>::findIndex<Scaleform::GFx::FontHandle *>");
    assert_eq!(result, vec![
        "Scaleform",
        "HashSetBase<Scaleform::GFx::FontManager::NodePtr,Scaleform::GFx::FontManager::NodePtrHashOp,Scaleform::GFx::FontManager::NodePtrHashOp,Scaleform::AllocatorLH<Scaleform::GFx::FontManager::NodePtr,2>,Scaleform::HashsetCachedEntry<Scaleform::GFx::FontManager::NodePtr,Scaleform::GFx::FontManager::NodePtrHashOp>>",
        "findIndex<Scaleform::GFx::FontHandle *>"
    ]);
}

#[test]
fn test_signature_253() {
    let result = parse_signature("void __fastcall Scaleform::Render::ShapeDataFloatTempl<Scaleform::ArrayLH_POD<unsigned char,2,Scaleform::ArrayDefaultPolicy>>::StartPath");
    assert_eq!(result, vec![
        "Scaleform",
        "Render",
        "ShapeDataFloatTempl<Scaleform::ArrayLH_POD<unsigned char,2,Scaleform::ArrayDefaultPolicy>>",
        "StartPath"
    ]);
}

#[test]
fn test_signature_254() {
    let result = parse_signature("void __fastcall Scaleform::Render::Text::LineBuffer::Line::SetParagraphModId");
    assert_eq!(result, vec![
        "Scaleform",
        "Render",
        "Text",
        "LineBuffer",
        "Line",
        "SetParagraphModId"
    ]);
}

#[test]
fn test_signature_255() {
    let result = parse_signature("unsigned int *__fastcall Scaleform::Render::Text::ParagraphFormat::GetTabStops");
    assert_eq!(result, vec![
        "Scaleform",
        "Render",
        "Text",
        "ParagraphFormat",
        "GetTabStops"
    ]);
}

#[test]
fn test_signature_256() {
    let result = parse_signature("Scaleform::Render::Tessellator::StarVertexType *__fastcall Scaleform::Render::ArrayPaged<Scaleform::Render::Tessellator::StarVertexType,4,16>::operator[]");
    assert_eq!(result, vec![
        "Scaleform",
        "Render",
        "ArrayPaged<Scaleform::Render::Tessellator::StarVertexType,4,16>",
        "operator[]"
    ]);
}

#[test]
fn test_signature_257() {
    let result = parse_signature("void __fastcall Scaleform::Render::MatrixState::Copy");
    assert_eq!(result, vec![
        "Scaleform",
        "Render",
        "MatrixState",
        "Copy"
    ]);
}

#[test]
fn test_signature_258() {
    let result = parse_signature("void __fastcall Scaleform::HashSetBase<Scaleform::HashNode<Scaleform::GFx::ResourceId,Scaleform::GFx::ResourcePtr<Scaleform::GFx::ImageResource>,Scaleform::FixedSizeHash<Scaleform::GFx::ResourceId>>,Scaleform::HashNode<Scaleform::GFx::ResourceId,Scaleform::GFx::ResourcePtr<Scaleform::GFx::ImageResource>,Scaleform::FixedSizeHash<Scaleform::GFx::ResourceId>>::NodeHashF,Scaleform::HashNode<Scaleform::GFx::ResourceId,Scaleform::GFx::ResourcePtr<Scaleform::GFx::ImageResource>,Scaleform::FixedSizeHash<Scaleform::GFx::ResourceId>>::NodeAltHashF,Scaleform::AllocatorLH<Scaleform::GFx::ResourceId,261>,Scaleform::HashsetCachedNodeEntry<Scaleform::HashNode<Scaleform::GFx::ResourceId,Scaleform::GFx::ResourcePtr<Scaleform::GFx::ImageResource>,Scaleform::FixedSizeHash<Scaleform::GFx::ResourceId>>,Scaleform::HashNode<Scaleform::GFx::ResourceId,Scaleform::GFx::ResourcePtr<Scaleform::GFx::ImageResource>,Scaleform::FixedSizeHash<Scaleform::GFx::ResourceId>>::NodeHashF>>::ConstIterator::operator++");
    assert_eq!(result, vec![
        "Scaleform",
        "HashSetBase<Scaleform::HashNode<Scaleform::GFx::ResourceId,Scaleform::GFx::ResourcePtr<Scaleform::GFx::ImageResource>,Scaleform::FixedSizeHash<Scaleform::GFx::ResourceId>>,Scaleform::HashNode<Scaleform::GFx::ResourceId,Scaleform::GFx::ResourcePtr<Scaleform::GFx::ImageResource>,Scaleform::FixedSizeHash<Scaleform::GFx::ResourceId>>::NodeHashF,Scaleform::HashNode<Scaleform::GFx::ResourceId,Scaleform::GFx::ResourcePtr<Scaleform::GFx::ImageResource>,Scaleform::FixedSizeHash<Scaleform::GFx::ResourceId>>::NodeAltHashF,Scaleform::AllocatorLH<Scaleform::GFx::ResourceId,261>,Scaleform::HashsetCachedNodeEntry<Scaleform::HashNode<Scaleform::GFx::ResourceId,Scaleform::GFx::ResourcePtr<Scaleform::GFx::ImageResource>,Scaleform::FixedSizeHash<Scaleform::GFx::ResourceId>>,Scaleform::HashNode<Scaleform::GFx::ResourceId,Scaleform::GFx::ResourcePtr<Scaleform::GFx::ImageResource>,Scaleform::FixedSizeHash<Scaleform::GFx::ResourceId>>::NodeHashF>>",
        "ConstIterator",
        "operator++"
    ]);
}

#[test]
fn test_signature_259() {
    let result = parse_signature("const Scaleform::Ptr<Scaleform::GFx::AMP::Message> *__fastcall Scaleform::Ptr<Scaleform::GFx::AMP::Message>::operator=");
    assert_eq!(result, vec![
        "Scaleform",
        "Ptr<Scaleform::GFx::AMP::Message>",
        "operator="
    ]);
}

#[test]
fn test_signature_260() {
    let result = parse_signature("const Scaleform::Render::RectPacker::RectType *__fastcall Scaleform::ArrayPagedBase<Scaleform::Render::RectPacker::RectType,8,64,Scaleform::AllocatorPagedLH_POD<Scaleform::Render::RectPacker::RectType,2>>::operator[]");
    assert_eq!(result, vec![
        "Scaleform",
        "ArrayPagedBase<Scaleform::Render::RectPacker::RectType,8,64,Scaleform::AllocatorPagedLH_POD<Scaleform::Render::RectPacker::RectType,2>>",
        "operator[]"
    ]);
}

#[test]
fn test_signature_261() {
    let result = parse_signature("void __fastcall Scaleform::GFx::TextField::GetInitialFormats");
    assert_eq!(result, vec![
        "Scaleform",
        "GFx",
        "TextField",
        "GetInitialFormats"
    ]);
}

#[test]
fn test_signature_262() {
    let result = parse_signature("bool __fastcall Scaleform::Render::ComplexMeshVertexOutput::BeginOutput");
    assert_eq!(result, vec![
        "Scaleform",
        "Render",
        "ComplexMeshVertexOutput",
        "BeginOutput"
    ]);
}

#[test]
fn test_signature_263() {
    let result = parse_signature("void __fastcall Scaleform::GFx::AS3::Instances::fl_sampler::DeleteObjectSample::DeleteObjectSample");
    assert_eq!(result, vec![
        "Scaleform",
        "GFx",
        "AS3",
        "Instances",
        "fl_sampler",
        "DeleteObjectSample",
        "DeleteObjectSample"
    ]);
}

#[test]
fn test_signature_264() {
    let result = parse_signature("__int64 __fastcall Scaleform::GFx::AS3::ClassInfo::GetInstanceMemberNum");
    assert_eq!(result, vec![
        "Scaleform",
        "GFx",
        "AS3",
        "ClassInfo",
        "GetInstanceMemberNum"
    ]);
}

#[test]
fn test_signature_265() {
    let result = parse_signature("__int64 __fastcall Scaleform::GFx::AS3::TR::NodeIF::GetOpCode");
    assert_eq!(result, vec![
        "Scaleform",
        "GFx",
        "AS3",
        "TR",
        "NodeIF",
        "GetOpCode"
    ]);
}

#[test]
fn test_signature_266() {
    let result = parse_signature("unsigned __int64 __fastcall Scaleform::ArrayDataBase<Scaleform::GFx::Value,Scaleform::AllocatorGH_CPP<Scaleform::GFx::Value,2>,Scaleform::ArrayDefaultPolicy>::GetCapacity");
    assert_eq!(result, vec![
        "Scaleform",
        "ArrayDataBase<Scaleform::GFx::Value,Scaleform::AllocatorGH_CPP<Scaleform::GFx::Value,2>,Scaleform::ArrayDefaultPolicy>",
        "GetCapacity"
    ]);
}

#[test]
fn test_signature_267() {
    let result = parse_signature("void __fastcall Scaleform::ArrayDataBase<Scaleform::GFx::AS3::Abc::NamespaceSetInfo,Scaleform::AllocatorLH_POD<Scaleform::GFx::AS3::Abc::NamespaceSetInfo,340>,Scaleform::ArrayDefaultPolicy>::~ArrayDataBase<Scaleform::GFx::AS3::Abc::NamespaceSetInfo,Scaleform::AllocatorLH_POD<Scaleform::GFx::AS3::Abc::NamespaceSetInfo,340>,Scaleform::ArrayDefaultPolicy>");
    assert_eq!(result, vec![
        "Scaleform",
        "ArrayDataBase<Scaleform::GFx::AS3::Abc::NamespaceSetInfo,Scaleform::AllocatorLH_POD<Scaleform::GFx::AS3::Abc::NamespaceSetInfo,340>,Scaleform::ArrayDefaultPolicy>",
        "~ArrayDataBase<Scaleform::GFx::AS3::Abc::NamespaceSetInfo,Scaleform::AllocatorLH_POD<Scaleform::GFx::AS3::Abc::NamespaceSetInfo,340>,Scaleform::ArrayDefaultPolicy>"
    ]);
}

#[test]
fn test_signature_268() {
    let result = parse_signature("unsigned __int64 __fastcall Scaleform::HashNode<Scaleform::GFx::AS3::SPtr<Scaleform::GFx::AS3::InstanceTraits::Traits>,unsigned __int64,Scaleform::FixedSizeHash<Scaleform::GFx::AS3::SPtr<Scaleform::GFx::AS3::InstanceTraits::Traits>>>::NodeAltHashF::operator()<Scaleform::GFx::AS3::SPtr<Scaleform::GFx::AS3::InstanceTraits::Traits>>");
    assert_eq!(result, vec![
        "Scaleform",
        "HashNode<Scaleform::GFx::AS3::SPtr<Scaleform::GFx::AS3::InstanceTraits::Traits>,unsigned __int64,Scaleform::FixedSizeHash<Scaleform::GFx::AS3::SPtr<Scaleform::GFx::AS3::InstanceTraits::Traits>>>",
        "NodeAltHashF",
        "operator()<Scaleform::GFx::AS3::SPtr<Scaleform::GFx::AS3::InstanceTraits::Traits>>"
    ]);
}

#[test]
fn test_signature_269() {
    let result = parse_signature("void __fastcall Scaleform::GFx::AS3::PropRef::PropRef");
    assert_eq!(result, vec![
        "Scaleform",
        "GFx",
        "AS3",
        "PropRef",
        "PropRef"
    ]);
}

#[test]
fn test_signature_270() {
    let result = parse_signature("void __fastcall Scaleform::ArrayDataBase<Scaleform::GFx::AS3::Slots::Pair,Scaleform::AllocatorLH<Scaleform::GFx::AS3::Slots::Pair,333>,Scaleform::ArrayDefaultPolicy>::ResizeNoConstruct");
    assert_eq!(result, vec![
        "Scaleform",
        "ArrayDataBase<Scaleform::GFx::AS3::Slots::Pair,Scaleform::AllocatorLH<Scaleform::GFx::AS3::Slots::Pair,333>,Scaleform::ArrayDefaultPolicy>",
        "ResizeNoConstruct"
    ]);
}

#[test]
fn test_signature_271() {
    let result = parse_signature("__int64 __fastcall Scaleform::GFx::AS3::VM::exec_iftype");
    assert_eq!(result, vec![
        "Scaleform",
        "GFx",
        "AS3",
        "VM",
        "exec_iftype"
    ]);
}

#[test]
fn test_signature_272() {
    let result = parse_signature("void __fastcall Scaleform::GFx::AS3::SPtr<Scaleform::GFx::AS3::Instances::fl::Object>::SPtr<Scaleform::GFx::AS3::Instances::fl::Object>");
    assert_eq!(result, vec![
        "Scaleform",
        "GFx",
        "AS3",
        "SPtr<Scaleform::GFx::AS3::Instances::fl::Object>",
        "SPtr<Scaleform::GFx::AS3::Instances::fl::Object>"
    ]);
}

#[test]
fn test_signature_273() {
    let result = parse_signature("void __fastcall Scaleform::GFx::AS3::Instances::fl_events::KeyboardEvent::SetEventId");
    assert_eq!(result, vec![
        "Scaleform",
        "GFx",
        "AS3",
        "Instances",
        "fl_events",
        "KeyboardEvent",
        "SetEventId"
    ]);
}

#[test]
fn test_signature_274() {
    let result = parse_signature("void __fastcall Scaleform::GFx::AS3::InstanceTraits::fl_events::TransformGestureEvent::MakeObject");
    assert_eq!(result, vec![
        "Scaleform",
        "GFx",
        "AS3",
        "InstanceTraits",
        "fl_events",
        "TransformGestureEvent",
        "MakeObject"
    ]);
}

#[test]
fn test_signature_275() {
    let result = parse_signature("char __fastcall Scaleform::GFx::AS3::ClassTraits::fl::Boolean::CoerceValue");
    assert_eq!(result, vec![
        "Scaleform",
        "GFx",
        "AS3",
        "ClassTraits",
        "fl",
        "Boolean",
        "CoerceValue"
    ]);
}

#[test]
fn test_signature_276() {
    let result = parse_signature("bool __fastcall Scaleform::GFx::AS3::Instances::fl::XMLElement::HasOwnProperty");
    assert_eq!(result, vec![
        "Scaleform",
        "GFx",
        "AS3",
        "Instances",
        "fl",
        "XMLElement",
        "HasOwnProperty"
    ]);
}

#[test]
fn test_signature_277() {
    let result = parse_signature("void __fastcall Scaleform::GFx::AS3::ThunkFunc1<Scaleform::GFx::AS3::Instances::fl_gfx::GamePadAnalogEvent,7,Scaleform::GFx::AS3::Value const,double>::Func");
    assert_eq!(result, vec![
        "Scaleform",
        "GFx",
        "AS3",
        "ThunkFunc1<Scaleform::GFx::AS3::Instances::fl_gfx::GamePadAnalogEvent,7,Scaleform::GFx::AS3::Value const,double>",
        "Func"
    ]);
}

#[test]
fn test_signature_278() {
    let result = parse_signature("const char *__fastcall Scaleform::GFx::AS3::ClassTraits::fl_text::AntiAliasType::GetAS3ObjectType");
    assert_eq!(result, vec![
        "Scaleform",
        "GFx",
        "AS3",
        "ClassTraits",
        "fl_text",
        "AntiAliasType",
        "GetAS3ObjectType"
    ]);
}

#[test]
fn test_signature_279() {
    let result = parse_signature("void __fastcall Scaleform::GFx::AS3::Instances::fl_filters::BevelFilter::shadowAlphaGet");
    assert_eq!(result, vec![
        "Scaleform",
        "GFx",
        "AS3",
        "Instances",
        "fl_filters",
        "BevelFilter",
        "shadowAlphaGet"
    ]);
}

#[test]
fn test_signature_280() {
    let result = parse_signature("__int64 __fastcall Scaleform::GFx::AS3::Instances::fl_utils::ByteArray::readUnsignedInt");
    assert_eq!(result, vec![
        "Scaleform",
        "GFx",
        "AS3",
        "Instances",
        "fl_utils",
        "ByteArray",
        "readUnsignedInt"
    ]);
}

#[test]
fn test_signature_281() {
    let result = parse_signature("void __fastcall Scaleform::GFx::AS3::ClassTraits::fl_display::GraphicsPathWinding::GraphicsPathWinding");
    assert_eq!(result, vec![
        "Scaleform",
        "GFx",
        "AS3",
        "ClassTraits",
        "fl_display",
        "GraphicsPathWinding",
        "GraphicsPathWinding"
    ]);
}

#[test]
fn test_signature_282() {
    let result = parse_signature("void __fastcall Scaleform::GFx::AS3::Instances::fl_vec::Vector_object::AS3every");
    assert_eq!(result, vec![
        "Scaleform",
        "GFx",
        "AS3",
        "Instances",
        "fl_vec",
        "Vector_object",
        "AS3every"
    ]);
}

#[test]
fn test_signature_283() {
    let result = parse_signature("unsigned __int8 *__fastcall png_get_io_chunk_name");
    assert_eq!(result, vec![
        "png_get_io_chunk_name"
    ]);
}

#[test]
fn test_signature_284() {
    let result = parse_signature("void __fastcall Scaleform::HashSetBase<Scaleform::HashNode<void const *,Scaleform::Ptr<ID3D11DeviceChild>,Scaleform::FixedSizeHash<void const *>>,Scaleform::HashNode<void const *,Scaleform::Ptr<ID3D11DeviceChild>,Scaleform::FixedSizeHash<void const *>>::NodeHashF,Scaleform::HashNode<void const *,Scaleform::Ptr<ID3D11DeviceChild>,Scaleform::FixedSizeHash<void const *>>::NodeAltHashF,Scaleform::AllocatorLH<void const *,2>,Scaleform::HashsetCachedNodeEntry<Scaleform::HashNode<void const *,Scaleform::Ptr<ID3D11DeviceChild>,Scaleform::FixedSizeHash<void const *>>,Scaleform::HashNode<void const *,Scaleform::Ptr<ID3D11DeviceChild>,Scaleform::FixedSizeHash<void const *>>::NodeHashF>>::HashSetBase<Scaleform::HashNode<void const *,Scaleform::Ptr<ID3D11DeviceChild>,Scaleform::FixedSizeHash<void const *>>,Scaleform::HashNode<void const *,Scaleform::Ptr<ID3D11DeviceChild>,Scaleform::FixedSizeHash<void const *>>::NodeHashF,Scaleform::HashNode<void const *,Scaleform::Ptr<ID3D11DeviceChild>,Scaleform::FixedSizeHash<void const *>>::NodeAltHashF,Scaleform::AllocatorLH<void const *,2>,Scaleform::HashsetCachedNodeEntry<Scaleform::HashNode<void const *,Scaleform::Ptr<ID3D11DeviceChild>,Scaleform::FixedSizeHash<void const *>>,Scaleform::HashNode<void const *,Scaleform::Ptr<ID3D11DeviceChild>,Scaleform::FixedSizeHash<void const *>>::NodeHashF>>");
    assert_eq!(result, vec![
        "Scaleform",
        "HashSetBase<Scaleform::HashNode<void const *,Scaleform::Ptr<ID3D11DeviceChild>,Scaleform::FixedSizeHash<void const *>>,Scaleform::HashNode<void const *,Scaleform::Ptr<ID3D11DeviceChild>,Scaleform::FixedSizeHash<void const *>>::NodeHashF,Scaleform::HashNode<void const *,Scaleform::Ptr<ID3D11DeviceChild>,Scaleform::FixedSizeHash<void const *>>::NodeAltHashF,Scaleform::AllocatorLH<void const *,2>,Scaleform::HashsetCachedNodeEntry<Scaleform::HashNode<void const *,Scaleform::Ptr<ID3D11DeviceChild>,Scaleform::FixedSizeHash<void const *>>,Scaleform::HashNode<void const *,Scaleform::Ptr<ID3D11DeviceChild>,Scaleform::FixedSizeHash<void const *>>::NodeHashF>>",
        "HashSetBase<Scaleform::HashNode<void const *,Scaleform::Ptr<ID3D11DeviceChild>,Scaleform::FixedSizeHash<void const *>>,Scaleform::HashNode<void const *,Scaleform::Ptr<ID3D11DeviceChild>,Scaleform::FixedSizeHash<void const *>>::NodeHashF,Scaleform::HashNode<void const *,Scaleform::Ptr<ID3D11DeviceChild>,Scaleform::FixedSizeHash<void const *>>::NodeAltHashF,Scaleform::AllocatorLH<void const *,2>,Scaleform::HashsetCachedNodeEntry<Scaleform::HashNode<void const *,Scaleform::Ptr<ID3D11DeviceChild>,Scaleform::FixedSizeHash<void const *>>,Scaleform::HashNode<void const *,Scaleform::Ptr<ID3D11DeviceChild>,Scaleform::FixedSizeHash<void const *>>::NodeHashF>>"
    ]);
}

#[test]
fn test_signature_285() {
    let result = parse_signature("void __fastcall CloseSocket");
    assert_eq!(result, vec![
        "CloseSocket"
    ]);
}

#[test]
fn test_signature_286() {
    let result = parse_signature("void __fastcall std::allocator<testing::internal::TraceInfo>::construct");
    assert_eq!(result, vec![
        "std",
        "allocator<testing::internal::TraceInfo>",
        "construct"
    ]);
}

#[test]
fn test_signature_287() {
    let result = parse_signature("soa8 *__fastcall operator/");
    assert_eq!(result, vec![
        "operator/"
    ]);
}

#[test]
fn test_signature_288() {
    let result = parse_signature("char *__fastcall HIKEffectorNameFromEffectorId");
    assert_eq!(result, vec![
        "HIKEffectorNameFromEffectorId"
    ]);
}

#[test]
fn test_signature_289() {
    let result = parse_signature("__m128 *__fastcall HIK2013::lps1<HIK2013::cg_y<float,__m128>>::operator=");
    assert_eq!(result, vec![
        "HIK2013",
        "lps1<HIK2013::cg_y<float,__m128>>",
        "operator="
    ]);
}

#[test]
fn test_signature_290() {
    let result = parse_signature("void __fastcall anonymous_namespace_::updateClassVersion1Inplace");
    assert_eq!(result, vec![
        "anonymous_namespace_",
        "updateClassVersion1Inplace"
    ]);
}

#[test]
fn test_signature_291() {
    let result = parse_signature("void __fastcall hkArray<hkGeometryUtils::Node::Edge,hkContainerHeapAllocator>::pushBack");
    assert_eq!(result, vec![
        "hkArray<hkGeometryUtils::Node::Edge,hkContainerHeapAllocator>",
        "pushBack"
    ]);
}

#[test]
fn test_signature_292() {
    let result = parse_signature("hkxMesh *__fastcall hkRefPtr<hkxMesh>::val");
    assert_eq!(result, vec![
        "hkRefPtr<hkxMesh>",
        "val"
    ]);
}

#[test]
fn test_signature_293() {
    let result = parse_signature("bool __fastcall hkgpConvexDecompositionImpl::Comparators::operator()");
    assert_eq!(result, vec![
        "hkgpConvexDecompositionImpl",
        "Comparators",
        "operator()"
    ]);
}

#[test]
fn test_signature_294() {
    let result = parse_signature("void __fastcall hkArrayBase<hknpHybridAabbTree<unsigned int,unsigned int,hkAabb16>::Node const *>::_setSize");
    assert_eq!(result, vec![
        "hkArrayBase<hknpHybridAabbTree<unsigned int,unsigned int,hkAabb16>::Node const *>",
        "_setSize"
    ]);
}

#[test]
fn test_signature_295() {
    let result = parse_signature("void __fastcall hkAlgorithm::quickSort<`anonymous namespace'::TreePairState,hkAlgorithm::less<`anonymous namespace'::TreePairState>>");
    assert_eq!(result, vec![
        "hkAlgorithm",
        "quickSort<`anonymous namespace'::TreePairState,hkAlgorithm::less<`anonymous namespace'::TreePairState>>"
    ]);
}

#[test]
fn test_signature_296() {
    let result = parse_signature("void __fastcall hkgpTriangulatorType<hkContainerHeapAllocator,hkgpTriangulatorBase::VertexBase,hkgpTriangulatorBase::TriangleBase,hkgpTriangulatorBase::DefaultEdgeData<hkContainerHeapAllocator>,hkgpTriangulatorBase::NoEdgeDataPolicy<hkgpTriangulatorBase::DefaultEdgeData<hkContainerHeapAllocator>,hkContainerHeapAllocator>,-1,4,23,0>::Insertion::Insertion");
    assert_eq!(result, vec![
        "hkgpTriangulatorType<hkContainerHeapAllocator,hkgpTriangulatorBase::VertexBase,hkgpTriangulatorBase::TriangleBase,hkgpTriangulatorBase::DefaultEdgeData<hkContainerHeapAllocator>,hkgpTriangulatorBase::NoEdgeDataPolicy<hkgpTriangulatorBase::DefaultEdgeData<hkContainerHeapAllocator>,hkContainerHeapAllocator>,-1,4,23,0>",
        "Insertion",
        "Insertion"
    ]);
}

#[test]
fn test_signature_297() {
    let result = parse_signature("void __fastcall hkRefPtr<hkdAction>::~hkRefPtr<hkdAction>");
    assert_eq!(result, vec![
        "hkRefPtr<hkdAction>",
        "~hkRefPtr<hkdAction>"
    ]);
}

#[test]
fn test_signature_298() {
    let result = parse_signature("hkResult *__fastcall hkArray<hkgpBooleanImpl::ISecPoint *,hkContainerHeapAllocator>::reserve");
    assert_eq!(result, vec![
        "hkArray<hkgpBooleanImpl::ISecPoint *,hkContainerHeapAllocator>",
        "reserve"
    ]);
}

#[test]
fn test_signature_299() {
    let result = parse_signature("void __fastcall hkArrayBase<hkdDecorateFractureFaceAction::CompressedDecorationPlacement>::hkArrayBase<hkdDecorateFractureFaceAction::CompressedDecorationPlacement>");
    assert_eq!(result, vec![
        "hkArrayBase<hkdDecorateFractureFaceAction::CompressedDecorationPlacement>",
        "hkArrayBase<hkdDecorateFractureFaceAction::CompressedDecorationPlacement>"
    ]);
}

#[test]
fn test_signature_300() {
    let result = parse_signature("void NGraphicsEngine::_dynamic_initializer_for__INV_LIGHT_MAX_SQ_DISTANCE___8");
    assert_eq!(result, vec![
        "void NGraphicsEngine",
        "_dynamic_initializer_for__INV_LIGHT_MAX_SQ_DISTANCE___8"
    ]);
}

#[test]
fn test_signature_301() {
    let result = parse_signature("__int64 NGameSoundLibrary::NGate::_dynamic_initializer_for__T_TIME_SPAN__NAME_HASH___8");
    assert_eq!(result, vec![
        "__int64 NGameSoundLibrary",
        "NGate",
        "_dynamic_initializer_for__T_TIME_SPAN__NAME_HASH___8"
    ]);
}

#[test]
fn test_signature_302() {
    let result = parse_signature("__int64 dynamic_initializer_for__hash_settings_menu_input_gamepad_ps4__");
    assert_eq!(result, vec![
        "__int64 dynamic_initializer_for__hash_settings_menu_input_gamepad_ps4__"
    ]);
}

#[test]
fn test_signature_303() {
    let result = parse_signature("const hkClassEnum *dynamic_initializer_for__hkaiAstarCostModifierClass_Members__");
    assert_eq!(result, vec![
        "const hkClassEnum *dynamic_initializer_for__hkaiAstarCostModifierClass_Members__"
    ]);
}

#[test]
fn test_signature_304() {
    let result = parse_signature("__int64 dynamic_initializer_for__Scaleform::GFx::AS3::ThunkFunc0_Scaleform::GFx::AS3::Instances::fl_media::Sound_3_bool_::Method__");
    assert_eq!(result, vec![
        "__int64 dynamic_initializer_for__Scaleform",
        "GFx",
        "AS3",
        "ThunkFunc0_Scaleform",
        "GFx",
        "AS3",
        "Instances",
        "fl_media",
        "Sound_3_bool_",
        "Method__"
    ]);
}

#[test]
fn test_signature_305() {
    let result = parse_signature("__int64 dynamic_initializer_for__Scaleform::GFx::AS3::ThunkFunc1_Scaleform::GFx::AS3::Instances::fl_filters::DropShadowFilter_17_Scaleform::GFx::AS3::Value_const__bool_::Method__");
    assert_eq!(result, vec![
        "__int64 dynamic_initializer_for__Scaleform",
        "GFx",
        "AS3",
        "ThunkFunc1_Scaleform",
        "GFx",
        "AS3",
        "Instances",
        "fl_filters",
        "DropShadowFilter_17_Scaleform",
        "GFx",
        "AS3",
        "Value_const__bool_",
        "Method__"
    ]);
}

#[test]
fn test_signature_306() {
    let result = parse_signature("void *Scaleform::Render::D3D1x::_dynamic_initializer_for__ShaderDesc_VS_D3D1xFL11X_VInstancedYUVCxform__");
    assert_eq!(result, vec![
        "void *Scaleform",
        "Render",
        "D3D1x",
        "_dynamic_initializer_for__ShaderDesc_VS_D3D1xFL11X_VInstancedYUVCxform__"
    ]);
}

#[test]
fn test_signature_307() {
    let result = parse_signature("__int64 Scaleform::Render::D3D1x::_dynamic_initializer_for__ShaderDesc_FS_D3D1xFL10X_FBatchTextMul__");
    assert_eq!(result, vec![
        "__int64 Scaleform",
        "Render",
        "D3D1x",
        "_dynamic_initializer_for__ShaderDesc_FS_D3D1xFL10X_FBatchTextMul__"
    ]);
}

#[test]
fn test_signature_308() {
    let result = parse_signature("void dynamic_initializer_for__hkpDashpotActionClass__");
    assert_eq!(result, vec![
        "void dynamic_initializer_for__hkpDashpotActionClass__"
    ]);
}

#[test]
fn test_signature_309() {
    let result = parse_signature("__int64 NCharacter::_dynamic_atexit_destructor_for__S_RUN_GRPL_FIRE_180L__");
    assert_eq!(result, vec![
        "__int64 NCharacter",
        "_dynamic_atexit_destructor_for__S_RUN_GRPL_FIRE_180L__"
    ]);
}

#[test]
fn test_signature_310() {
    let result = parse_signature("void dynamic_atexit_destructor_for__s_wingsuit_evade_reticle_align__");
    assert_eq!(result, vec![
        "void dynamic_atexit_destructor_for__s_wingsuit_evade_reticle_align__"
    ]);
}

#[test]
fn test_signature_311() {
    let result = parse_signature("__int64 dynamic_atexit_destructor_for__hash_mission_b___0");
    assert_eq!(result, vec![
        "__int64 dynamic_atexit_destructor_for__hash_mission_b___0"
    ]);
}

