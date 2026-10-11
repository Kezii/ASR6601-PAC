#[doc = "Register `CR2` reader"]
pub type R = crate::R<Cr2Spec>;
#[doc = "Register `CR2` writer"]
pub type W = crate::W<Cr2Spec>;
#[doc = "Field `CCDS` reader - Channel DMA request source selection"]
pub type CcdsR = crate::BitReader;
#[doc = "Field `CCDS` writer - Channel DMA request source selection"]
pub type CcdsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Master mode selection, TRGO output"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mms {
    #[doc = "0: UG output as TRGO"]
    Reset = 0,
    #[doc = "1: CNT_EN output as TRGO"]
    Enable = 1,
    #[doc = "2: Update event output as TRGO"]
    Update = 2,
    #[doc = "3: Pulse on CC0IF about to set"]
    ComparePulse = 3,
    #[doc = "4: OC0REF output as TRGO"]
    CompareOc0ref = 4,
    #[doc = "5: OC1REF output as TRGO"]
    CompareOc1ref = 5,
    #[doc = "6: OC2REF output as TRGO"]
    CompareOc2ref = 6,
    #[doc = "7: OC3REF output as TRGO"]
    CompareOc3ref = 7,
}
impl From<Mms> for u8 {
    #[inline(always)]
    fn from(variant: Mms) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mms {
    type Ux = u8;
}
impl crate::IsEnum for Mms {}
#[doc = "Field `MMS` reader - Master mode selection, TRGO output"]
pub type MmsR = crate::FieldReader<Mms>;
impl MmsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mms {
        match self.bits {
            0 => Mms::Reset,
            1 => Mms::Enable,
            2 => Mms::Update,
            3 => Mms::ComparePulse,
            4 => Mms::CompareOc0ref,
            5 => Mms::CompareOc1ref,
            6 => Mms::CompareOc2ref,
            7 => Mms::CompareOc3ref,
            _ => unreachable!(),
        }
    }
    #[doc = "UG output as TRGO"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == Mms::Reset
    }
    #[doc = "CNT_EN output as TRGO"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Mms::Enable
    }
    #[doc = "Update event output as TRGO"]
    #[inline(always)]
    pub fn is_update(&self) -> bool {
        *self == Mms::Update
    }
    #[doc = "Pulse on CC0IF about to set"]
    #[inline(always)]
    pub fn is_compare_pulse(&self) -> bool {
        *self == Mms::ComparePulse
    }
    #[doc = "OC0REF output as TRGO"]
    #[inline(always)]
    pub fn is_compare_oc0ref(&self) -> bool {
        *self == Mms::CompareOc0ref
    }
    #[doc = "OC1REF output as TRGO"]
    #[inline(always)]
    pub fn is_compare_oc1ref(&self) -> bool {
        *self == Mms::CompareOc1ref
    }
    #[doc = "OC2REF output as TRGO"]
    #[inline(always)]
    pub fn is_compare_oc2ref(&self) -> bool {
        *self == Mms::CompareOc2ref
    }
    #[doc = "OC3REF output as TRGO"]
    #[inline(always)]
    pub fn is_compare_oc3ref(&self) -> bool {
        *self == Mms::CompareOc3ref
    }
}
#[doc = "Field `MMS` writer - Master mode selection, TRGO output"]
pub type MmsW<'a, REG> = crate::FieldWriter<'a, REG, 3, Mms, crate::Safe>;
impl<'a, REG> MmsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "UG output as TRGO"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(Mms::Reset)
    }
    #[doc = "CNT_EN output as TRGO"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Mms::Enable)
    }
    #[doc = "Update event output as TRGO"]
    #[inline(always)]
    pub fn update(self) -> &'a mut crate::W<REG> {
        self.variant(Mms::Update)
    }
    #[doc = "Pulse on CC0IF about to set"]
    #[inline(always)]
    pub fn compare_pulse(self) -> &'a mut crate::W<REG> {
        self.variant(Mms::ComparePulse)
    }
    #[doc = "OC0REF output as TRGO"]
    #[inline(always)]
    pub fn compare_oc0ref(self) -> &'a mut crate::W<REG> {
        self.variant(Mms::CompareOc0ref)
    }
    #[doc = "OC1REF output as TRGO"]
    #[inline(always)]
    pub fn compare_oc1ref(self) -> &'a mut crate::W<REG> {
        self.variant(Mms::CompareOc1ref)
    }
    #[doc = "OC2REF output as TRGO"]
    #[inline(always)]
    pub fn compare_oc2ref(self) -> &'a mut crate::W<REG> {
        self.variant(Mms::CompareOc2ref)
    }
    #[doc = "OC3REF output as TRGO"]
    #[inline(always)]
    pub fn compare_oc3ref(self) -> &'a mut crate::W<REG> {
        self.variant(Mms::CompareOc3ref)
    }
}
#[doc = "Field `TI0S` reader - Channel 0 source XOR selection"]
pub type Ti0sR = crate::BitReader;
#[doc = "Field `TI0S` writer - Channel 0 source XOR selection"]
pub type Ti0sW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 3 - Channel DMA request source selection"]
    #[inline(always)]
    pub fn ccds(&self) -> CcdsR {
        CcdsR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Master mode selection, TRGO output"]
    #[inline(always)]
    pub fn mms(&self) -> MmsR {
        MmsR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Channel 0 source XOR selection"]
    #[inline(always)]
    pub fn ti0s(&self) -> Ti0sR {
        Ti0sR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - Channel DMA request source selection"]
    #[inline(always)]
    pub fn ccds(&mut self) -> CcdsW<'_, Cr2Spec> {
        CcdsW::new(self, 3)
    }
    #[doc = "Bits 4:6 - Master mode selection, TRGO output"]
    #[inline(always)]
    pub fn mms(&mut self) -> MmsW<'_, Cr2Spec> {
        MmsW::new(self, 4)
    }
    #[doc = "Bit 7 - Channel 0 source XOR selection"]
    #[inline(always)]
    pub fn ti0s(&mut self) -> Ti0sW<'_, Cr2Spec> {
        Ti0sW::new(self, 7)
    }
}
#[doc = "GPTIM control register 2, Address\n\nYou can [`read`](crate::Reg::read) this register and get [`cr2::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cr2Spec;
impl crate::RegisterSpec for Cr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr2::R`](R) reader structure"]
impl crate::Readable for Cr2Spec {}
#[doc = "`write(|w| ..)` method takes [`cr2::W`](W) writer structure"]
impl crate::Writable for Cr2Spec {
    type Safety = crate::Unsafe;
}
