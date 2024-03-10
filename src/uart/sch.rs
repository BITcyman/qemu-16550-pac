#[doc = "Register `sch` reader"]
pub type R = crate::R<SchSpec>;
#[doc = "Register `sch` writer"]
pub type W = crate::W<SchSpec>;
#[doc = "Field `scratch` reader - "]
pub type ScratchR = crate::FieldReader;
#[doc = "Field `scratch` writer - "]
pub type ScratchW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn scratch(&self) -> ScratchR {
        ScratchR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn scratch(&mut self) -> ScratchW<SchSpec> {
        ScratchW::new(self, 0)
    }
}
#[doc = "UART Scratch Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sch::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sch::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SchSpec;
impl crate::RegisterSpec for SchSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`sch::R`](R) reader structure"]
impl crate::Readable for SchSpec {}
#[doc = "`write(|w| ..)` method takes [`sch::W`](W) writer structure"]
impl crate::Writable for SchSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets sch to value 0"]
impl crate::Resettable for SchSpec {
    const RESET_VALUE: u8 = 0;
}
